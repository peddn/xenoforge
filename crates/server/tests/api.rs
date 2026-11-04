//! The HTTP API end to end, with a fresh SQLite file per test and fixed
//! dice.

use std::path::PathBuf;
use std::time::Duration;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;
use xenoforge_engine::dice::ScriptedDice;
use xenoforge_server::{AppState, Config};

struct Server {
    app: Router,
    state: AppState,
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

async fn server(name: &str) -> Server {
    let dir = std::env::temp_dir().join(format!("xenoforge-api-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("index.html"),
        "<!doctype html><title>index</title>",
    )
    .unwrap();
    std::fs::create_dir_all(dir.join("assets")).unwrap();
    std::fs::write(dir.join("assets/index-abc123.js"), "// built").unwrap();
    let config = Config {
        database: dir.join("test.db"),
        static_dir: dir.clone(),
        data_dir: root().join("data"),
        secure_cookies: false,
    };
    let state = xenoforge_server::state(&config).await.unwrap();
    let app = xenoforge_server::router(state.clone(), &config.static_dir, &config.data_dir);
    Server { app, state }
}

impl Server {
    fn dice(&self, faces: &[u8]) {
        *self.state.dice.lock().unwrap() = Box::new(ScriptedDice::new(faces.iter().copied()));
    }
}

/// A browser: keeps the session cookie.
struct Client<'s> {
    server: &'s Server,
    cookie: Option<String>,
}

impl<'s> Client<'s> {
    fn new(server: &'s Server) -> Self {
        Self {
            server,
            cookie: None,
        }
    }

    async fn call(&mut self, method: &str, path: &str, body: Option<Value>) -> (StatusCode, Value) {
        let mut request = Request::builder().method(method).uri(path);
        if let Some(cookie) = &self.cookie {
            request = request.header(header::COOKIE, cookie);
        }
        let request = match body {
            Some(body) => request
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string())),
            None => request.body(Body::empty()),
        }
        .unwrap();
        let response = self.server.app.clone().oneshot(request).await.unwrap();
        if let Some(set) = response.headers().get(header::SET_COOKIE) {
            let pair = set.to_str().unwrap().split(';').next().unwrap().to_owned();
            self.cookie = Some(pair);
        }
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        (status, value)
    }

    async fn ok(&mut self, method: &str, path: &str, body: Option<Value>) -> Value {
        let (status, value) = self.call(method, path, body).await;
        assert!(status.is_success(), "{method} {path}: {status} {value}");
        value
    }

    async fn register(&mut self, name: &str, role: &str) -> i64 {
        let me = self
            .ok(
                "POST",
                "/api/auth/register",
                Some(json!({ "name": name, "password": "secret-password", "role": role })),
            )
            .await;
        me["id"].as_i64().unwrap()
    }

    /// Creates a character with the given values.
    async fn character(&mut self, name: &str, actor: Value) -> i64 {
        let created = self
            .ok("POST", "/api/characters", Some(json!({ "name": name })))
            .await;
        let id = created["id"].as_i64().unwrap();
        self.ok(
            "PUT",
            &format!("/api/characters/{id}"),
            Some(json!({ "name": name, "data": { "actor": actor } })),
        )
        .await;
        id
    }

    async fn command(&mut self, team: i64, command: Value) -> (StatusCode, Value) {
        self.call(
            "POST",
            &format!("/api/teams/{team}/commands"),
            Some(json!({ "command": command })),
        )
        .await
    }
}

/// Game master with a team, player with a character in it.
async fn table(server: &Server) -> (Client<'_>, Client<'_>, i64, i64) {
    let mut gm = Client::new(server);
    gm.register("Mother", "gm").await;
    let team = gm
        .ok(
            "POST",
            "/api/teams",
            Some(json!({ "name": "Hope's Last Day", "mode": "cinematic" })),
        )
        .await["id"]
        .as_i64()
        .unwrap();
    let token = gm.ok("GET", &format!("/api/teams/{team}"), None).await["invite_token"]
        .as_str()
        .unwrap()
        .to_owned();
    let mut player = Client::new(server);
    player.register("Gina", "player").await;
    let character = player
        .character(
            "Charlize",
            json!({ "attributes": { "strength": 3, "wits": 4 }, "skills": { "observation": 2 }, "health": 3 }),
        )
        .await;
    player
        .ok(
            "POST",
            &format!("/api/join/{token}"),
            Some(json!({ "character": character })),
        )
        .await;
    (gm, player, team, character)
}

fn events_of(entries: &Value) -> Vec<Value> {
    entries
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["entry"]["type"] == "engine")
        .map(|e| e["entry"]["event"].clone())
        .collect()
}

#[tokio::test]
async fn health_and_frontend() {
    let server = server("health").await;
    let mut client = Client::new(&server);
    let health = client.ok("GET", "/api/health", None).await;
    assert_eq!(health["status"], "ok");
    let (status, _) = client.call("GET", "/api/nothing", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let response = server
        .app
        .clone()
        .oneshot(Request::get("/characters/7").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let rules = client.ok("GET", "/api/data/rules/skills.json", None).await;
    assert_eq!(rules["entries"].as_array().unwrap().len(), 12);

    // The page and the rule data are checked on every load; built files
    // with a hash in their name are kept.
    for (path, expected) in [
        ("/", Some("no-cache")),
        ("/teams/3", Some("no-cache")),
        ("/api/data/rules/skills.json", Some("no-cache")),
        (
            "/assets/index-abc123.js",
            Some("public, max-age=31536000, immutable"),
        ),
        ("/api/health", None),
    ] {
        let response = server
            .app
            .clone()
            .oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        let cache = response
            .headers()
            .get(header::CACHE_CONTROL)
            .map(|v| v.to_str().unwrap());
        assert_eq!(cache, expected, "{path}");
    }
}

#[tokio::test]
async fn accounts() {
    let server = server("accounts").await;
    let mut a = Client::new(&server);
    let (status, _) = a.call("GET", "/api/auth/me", None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    a.register("Ripley", "player").await;
    assert_eq!(a.ok("GET", "/api/auth/me", None).await["name"], "Ripley");

    let mut b = Client::new(&server);
    let (status, _) = b
        .call(
            "POST",
            "/api/auth/register",
            Some(json!({ "name": "ripley", "password": "another-password", "role": "gm" })),
        )
        .await;
    assert_eq!(
        status,
        StatusCode::CONFLICT,
        "names are unique, ignoring case"
    );
    let (status, _) = b
        .call(
            "POST",
            "/api/auth/login",
            Some(json!({ "name": "Ripley", "password": "wrong-password" })),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    b.ok(
        "POST",
        "/api/auth/login",
        Some(json!({ "name": "Ripley", "password": "secret-password" })),
    )
    .await;
    b.ok("POST", "/api/auth/logout", None).await;
    let (status, _) = b.call("GET", "/api/auth/me", None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

/// Only game master accounts create teams; nobody sees foreign teams
/// (A-07).
#[tokio::test]
async fn teams_and_rights() {
    let server = server("teams").await;
    let mut player = Client::new(&server);
    player.register("Dallas", "player").await;
    let (status, _) = player
        .call(
            "POST",
            "/api/teams",
            Some(json!({ "name": "Nostromo", "mode": "campaign" })),
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    let (mut gm, mut member, team, character) = table(&server).await;
    let mut other_gm = Client::new(&server);
    other_gm.register("Weyland", "gm").await;
    let (status, _) = other_gm
        .call("GET", &format!("/api/teams/{team}"), None)
        .await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "another game master sees nothing"
    );

    let view = member.ok("GET", &format!("/api/teams/{team}"), None).await;
    assert!(view["invite_token"].is_null());
    assert_eq!(
        view["characters"][0]["actor"]["attributes"]["wits"], 4,
        "owners see their values"
    );

    let mut third = Client::new(&server);
    third.register("Hayes", "player").await;
    let token = gm.ok("GET", &format!("/api/teams/{team}"), None).await["invite_token"]
        .as_str()
        .unwrap()
        .to_owned();
    let hayes = third.character("Hayes", json!({})).await;
    third
        .ok(
            "POST",
            &format!("/api/join/{token}"),
            Some(json!({ "character": hayes })),
        )
        .await;
    let view = third.ok("GET", &format!("/api/teams/{team}"), None).await;
    let charlize = view["characters"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == character)
        .unwrap();
    assert!(
        charlize["actor"].is_null(),
        "players do not see other sheets (A-01)"
    );

    let (status, _) = member
        .call(
            "POST",
            &format!("/api/join/{token}"),
            Some(json!({ "character": character })),
        )
        .await;
    assert_eq!(
        status,
        StatusCode::CONFLICT,
        "one team per character (A-04)"
    );
    let (status, _) = member
        .call("DELETE", &format!("/api/characters/{character}"), None)
        .await;
    assert_eq!(status, StatusCode::CONFLICT);

    gm.ok(
        "DELETE",
        &format!("/api/teams/{team}/characters/{character}"),
        None,
    )
    .await;
    // Without a character in the team the player no longer sees it.
    let (status, _) = member
        .call("GET", &format!("/api/teams/{team}"), None)
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    member
        .ok("DELETE", &format!("/api/characters/{character}"), None)
        .await;
}

/// A player rolls for their own character and decides about the push;
/// others' characters are off limits.
#[tokio::test]
async fn rolls_and_decisions() {
    let server = server("rolls").await;
    let (mut gm, mut player, team, character) = table(&server).await;
    let actor = character.to_string();

    let preview = player
        .ok(
            "POST",
            &format!("/api/teams/{team}/preview"),
            Some(json!({ "actor": actor, "kind": { "skill": { "skill": "observation" } }, "helpers": 1 })),
        )
        .await;
    assert_eq!(preview["base_dice"], 7);

    server.dice(&[1, 2, 3, 4, 5, 2]);
    let roll = json!({ "command": "roll", "request": { "actor": actor, "kind": { "skill": { "skill": "observation" } } } });
    let (status, entries) = player.command(team, roll.clone()).await;
    assert_eq!(status, StatusCode::OK, "{entries}");
    // One command, one block: every entry names the first one.
    let list = entries.as_array().unwrap();
    assert!(list.len() > 1);
    assert!(
        list.iter().all(|e| e["batch"] == list[0]["id"]),
        "{entries}"
    );
    let events = events_of(&entries);
    let decision = events.iter().find(|e| e["event"] == "decision").unwrap();
    let pending = decision["pending"].as_u64().unwrap();

    let view = player.ok("GET", &format!("/api/teams/{team}"), None).await;
    assert_eq!(view["pending"][0]["options"], json!(["push", "accept"]));
    // The decision comes with the roll so far.
    let thrown = &view["pending"][0]["roll"];
    assert_eq!(thrown["skill"], "observation");
    assert_eq!(thrown["dice"].as_array().unwrap().len(), 6);
    assert_eq!(thrown["successes"], 0);
    assert_eq!(thrown["pushes_left"], 1);

    let (status, _) = gm
        .command(
            team,
            json!({ "command": "decide", "pending": pending, "choice": "accept" }),
        )
        .await;
    assert_eq!(status, StatusCode::OK, "the game master may decide too");

    // A second player cannot roll for Charlize.
    let mut stranger = Client::new(&server);
    stranger.register("Burke", "player").await;
    let token = gm.ok("GET", &format!("/api/teams/{team}"), None).await["invite_token"]
        .as_str()
        .unwrap()
        .to_owned();
    let burke = stranger.character("Burke", json!({})).await;
    stranger
        .ok(
            "POST",
            &format!("/api/join/{token}"),
            Some(json!({ "character": burke })),
        )
        .await;
    let (status, _) = stranger.command(team, roll).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let (status, _) = stranger
        .command(
            team,
            json!({ "command": "damage", "actor": actor, "amount": 1 }),
        )
        .await;
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "damage is for the game master"
    );
}

#[tokio::test]
async fn rule_errors_are_reported() {
    let server = server("errors").await;
    let (mut gm, _, team, _) = table(&server).await;
    let (status, body) = gm
        .command(
            team,
            json!({ "command": "decide", "pending": 99, "choice": "accept" }),
        )
        .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"], "rule");
}

/// Players roll without difficulty; modifications are checked (A-08).
#[tokio::test]
async fn roll_modifications_follow_a08() {
    let server = server("a08").await;
    let (mut gm, mut player, team, character) = table(&server).await;
    let actor = character.to_string();
    let preview = format!("/api/teams/{team}/preview");
    let skill = |modifiers: Value| json!({ "actor": actor, "kind": { "skill": { "skill": "observation" } }, "modifiers": modifiers, "helpers": 1 });

    let ok = player
        .ok(
            "POST",
            &preview,
            Some(skill(json!([{ "source": "gear", "value": 2 }]))),
        )
        .await;
    assert_eq!(
        ok["base_dice"], 9,
        "WITS 4 + OBSERVATION 2 + gear 2 + help 1"
    );

    for (who, body, status) in [
        (
            "player",
            skill(json!([{ "source": "difficulty_hard", "value": -2 }])),
            StatusCode::FORBIDDEN,
        ),
        (
            "player",
            json!({ "actor": actor, "kind": { "fixed_pool": { "dice": 200 } } }),
            StatusCode::FORBIDDEN,
        ),
        (
            "player",
            skill(json!([{ "source": "gear", "value": 4 }])),
            StatusCode::BAD_REQUEST,
        ),
        (
            "player",
            skill(json!([{ "source": "GM approved", "value": 50 }])),
            StatusCode::BAD_REQUEST,
        ),
        (
            "gm",
            skill(json!([{ "source": "difficulty_hard", "value": -1 }])),
            StatusCode::BAD_REQUEST,
        ),
        (
            "gm",
            skill(json!([{ "source": "gear", "value": 1 }, { "source": "gear", "value": 1 }])),
            StatusCode::BAD_REQUEST,
        ),
    ] {
        let client = if who == "gm" { &mut gm } else { &mut player };
        let (got, body_text) = client.call("POST", &preview, Some(body.clone())).await;
        assert_eq!(got, status, "{who}: {body} -> {body_text}");
        let (got, _) = client
            .command(team, json!({ "command": "roll", "request": body }))
            .await;
        assert_eq!(got, status, "{who} command");
    }

    let hard = gm
        .ok(
            "POST",
            &preview,
            Some(skill(json!([{ "source": "difficulty_hard", "value": -2 }]))),
        )
        .await;
    assert_eq!(hard["base_dice"], 5, "WITS 4 + OBSERVATION 2 - 2 + help 1");
    let view = player.ok("GET", &format!("/api/teams/{team}"), None).await;
    assert_eq!(view["characters"][0]["broken"], false);
    assert_eq!(view["characters"][0]["health_max"], 3);
}

/// A note travels with its command; players can leave with their own
/// character but not take out anyone else's.
#[tokio::test]
async fn notes_and_leaving() {
    let server = server("leave").await;
    let (mut gm, mut player, team, character) = table(&server).await;
    let actor = character.to_string();

    server.dice(&[1, 2, 3, 4, 5, 2]);
    let (status, entries) = player
        .call(
            "POST",
            &format!("/api/teams/{team}/commands"),
            Some(json!({
                "command": { "command": "roll", "request": { "actor": actor, "kind": { "skill": { "skill": "observation" } } } },
                "note": "  Tür zum Labor aufbrechen  "
            })),
        )
        .await;
    assert_eq!(status, StatusCode::OK, "{entries}");
    let list = entries.as_array().unwrap();
    assert_eq!(list[0]["note"], "Tür zum Labor aufbrechen");
    assert!(list[1..].iter().all(|e| e.get("note").is_none()));
    let (status, _) = gm
        .call(
            "POST",
            &format!("/api/teams/{team}/commands"),
            Some(json!({ "command": { "command": "end_session" }, "note": "x".repeat(201) })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    // Another player in the team cannot take Charlize out.
    let mut other = Client::new(&server);
    other.register("Burke", "player").await;
    let own = other
        .character("Burke", json!({ "attributes": { "strength": 3 } }))
        .await;
    let token = gm.ok("GET", &format!("/api/teams/{team}"), None).await["invite_token"]
        .as_str()
        .unwrap()
        .to_owned();
    other
        .ok(
            "POST",
            &format!("/api/join/{token}"),
            Some(json!({ "character": own })),
        )
        .await;
    let (status, _) = other
        .call(
            "DELETE",
            &format!("/api/teams/{team}/characters/{character}"),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    // Gina leaves with Charlize.
    let entries = player
        .ok(
            "DELETE",
            &format!("/api/teams/{team}/characters/{character}"),
            None,
        )
        .await;
    assert_eq!(entries[0]["entry"]["type"], "character_left");
    let (status, _) = player
        .call("GET", &format!("/api/teams/{team}"), None)
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

/// Corrections and commands can be undone (plan section 3 and 10).
#[tokio::test]
async fn correction_and_undo() {
    let server = server("undo").await;
    let (mut gm, mut player, team, character) = table(&server).await;
    let actor = character.to_string();
    gm.ok(
        "PUT",
        &format!("/api/teams/{team}/characters/{character}"),
        Some(json!({ "actor": { "attributes": { "strength": 3, "wits": 4 }, "health": 3, "stress": 4 } })),
    )
    .await;
    let stress = |view: &Value| view["characters"][0]["actor"]["stress"].clone();
    assert_eq!(
        stress(&player.ok("GET", &format!("/api/teams/{team}"), None).await),
        4
    );

    gm.command(
        team,
        json!({ "command": "change_stress", "actors": [actor], "delta": 1 }),
    )
    .await;
    assert_eq!(
        stress(&player.ok("GET", &format!("/api/teams/{team}"), None).await),
        5
    );
    gm.ok("POST", &format!("/api/teams/{team}/undo"), None)
        .await;
    assert_eq!(
        stress(&player.ok("GET", &format!("/api/teams/{team}"), None).await),
        4
    );
    gm.ok("POST", &format!("/api/teams/{team}/undo"), None)
        .await;
    assert_eq!(
        stress(&player.ok("GET", &format!("/api/teams/{team}"), None).await),
        0
    );
    let (status, _) = player
        .call("POST", &format!("/api/teams/{team}/undo"), None)
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    // Undo takes back values, not the player's notes.
    gm.command(
        team,
        json!({ "command": "change_stress", "actors": [actor], "delta": 1 }),
    )
    .await;
    player
        .ok(
            "PUT",
            &format!("/api/characters/{character}"),
            Some(
                json!({ "name": "Charlize", "data": { "sheet": { "notes": "Look for Torres" } } }),
            ),
        )
        .await;
    gm.ok("POST", &format!("/api/teams/{team}/undo"), None)
        .await;
    let sheet = player
        .ok("GET", &format!("/api/characters/{character}"), None)
        .await;
    assert_eq!(sheet["data"]["sheet"]["notes"], "Look for Torres");
    assert_eq!(sheet["data"]["actor"]["stress"], 0);

    // Removing a character ends the undo history: older states would
    // still refer to it.
    gm.command(
        team,
        json!({ "command": "change_stress", "actors": [actor], "delta": 1 }),
    )
    .await;
    gm.ok(
        "DELETE",
        &format!("/api/teams/{team}/characters/{character}"),
        None,
    )
    .await;
    let (status, _) = gm
        .call("POST", &format!("/api/teams/{team}/undo"), None)
        .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

/// The event stream replays the log from the start.
#[tokio::test]
async fn event_stream_replays_the_log() {
    let server = server("sse").await;
    let (mut gm, mut player, team, character) = table(&server).await;
    player
        .command(
            team,
            json!({ "command": "use_signature_item", "actor": character.to_string() }),
        )
        .await;
    let mut request = Request::get(format!("/api/teams/{team}/events"))
        .body(Body::empty())
        .unwrap();
    request.headers_mut().insert(
        header::COOKIE,
        player.cookie.clone().unwrap().parse().unwrap(),
    );
    let response = server.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let mut body = response.into_body();
    let mut text = String::new();
    while !text.contains("character_joined") {
        let frame = tokio::time::timeout(Duration::from_secs(2), body.frame())
            .await
            .expect("an event arrives")
            .unwrap()
            .unwrap();
        if let Some(data) = frame.data_ref() {
            text.push_str(std::str::from_utf8(data).unwrap());
        }
    }
    assert!(text.contains("id: 1"), "{text}");

    // When a character leaves, the stream ends after telling so.
    gm.ok(
        "DELETE",
        &format!("/api/teams/{team}/characters/{character}"),
        None,
    )
    .await;
    while let Some(frame) = tokio::time::timeout(Duration::from_secs(2), body.frame())
        .await
        .expect("the stream ends")
    {
        if let Some(data) = frame.unwrap().data_ref() {
            text.push_str(std::str::from_utf8(data).unwrap());
        }
    }
    assert!(text.contains("character_removed"), "{text}");
}

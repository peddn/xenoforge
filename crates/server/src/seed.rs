//! Sample data for trying the app out: `xenoforge-server --seed` fills an
//! empty database with accounts, characters and two teams, then the server
//! starts as usual.
//!
//! Everything goes through the API in-process (`local`), so the sample
//! data passes the same checks as in play: the wizard, finishing, joining,
//! offering. Names and texts are made up; only Nat uses the numbers of the
//! book's example [p. 28].

use axum::Router;
use serde_json::{Value, json};
use sqlx::{Row, SqlitePool};

use crate::local::Client;

/// The password of every sample account.
pub const PASSWORD: &str = "xenoforge-test";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Fills an empty database and says what it created; refuses a database
/// that has accounts already, so real data is never mixed with samples.
pub async fn seed(app: Router, pool: &SqlitePool) -> Result<Vec<String>> {
    let users: i64 = sqlx::query("SELECT COUNT(*) AS n FROM users")
        .fetch_one(pool)
        .await?
        .get("n");
    if users > 0 {
        return Err(
            "--seed fills an empty database only; delete it or choose another XENOFORGE_DB".into(),
        );
    }

    let mut gm = account(&app, "Mother", "gm").await?;
    let mut ann = account(&app, "Ann", "player").await?;
    let mut bob = account(&app, "Bob", "player").await?;
    let mut cid = account(&app, "Cid", "player").await?;

    // Campaign: two characters made with the wizard, buddy and rival.
    let nat = finished(&mut ann, nat()).await?;
    let varga = finished(&mut bob, varga()).await?;
    let draft = ann.ok("POST", "/api/characters", Some(json!({}))).await?["id"].clone();
    ann.ok(
        "PUT",
        &format!("/api/characters/{draft}"),
        Some(json!({ "data": { "draft": {
            "career": "medic",
            "attributes": { "strength": 3, "agility": 3, "wits": 2, "empathy": 4 }
        } } })),
    )
    .await?;
    let campaign = team(&mut gm, "Testlauf Kampagne", "campaign").await?;
    join(&mut ann, &campaign, Some(&nat)).await?;
    join(&mut bob, &campaign, Some(&varga)).await?;
    ann.ok(
        "PUT",
        &format!("/api/characters/{nat}/relations"),
        Some(json!({ "buddy": varga, "rival": null })),
    )
    .await?;
    bob.ok(
        "PUT",
        &format!("/api/characters/{varga}/relations"),
        Some(json!({ "buddy": null, "rival": nat })),
    )
    .await?;
    // Something to eat and drink for Nat [p. 137].
    for item in [
        json!({ "item": "beer", "count": 2 }),
        json!({ "item": "prefab_meal" }),
    ] {
        gm.ok(
            "POST",
            &format!("/api/teams/{}/commands", campaign.id),
            Some(json!({ "command": { "command": "add_item", "actor": nat.to_string(), "item": item } })),
        )
        .await?;
    }

    // Cinematic: three prepared characters on offer, one a covert android,
    // each with an Agenda per Act; Ann and Cid join to choose.
    let scenario = team(&mut gm, "Testlauf Szenario", "cinematic").await?;
    let id = &scenario.id;
    for (name, career, actor, agendas, items) in prepared() {
        let created = gm
            .ok(
                "POST",
                "/api/characters",
                Some(json!({ "free": true, "name": name })),
            )
            .await?["id"]
            .clone();
        gm.ok(
            "PUT",
            &format!("/api/characters/{created}"),
            Some(
                json!({ "name": name, "data": { "sheet": { "career": career }, "actor": actor } }),
            ),
        )
        .await?;
        for item in items {
            gm.ok(
                "POST",
                &format!("/api/characters/{created}/items"),
                Some(json!({ "command": "add_item", "item": item })),
            )
            .await?;
        }
        gm.ok(
            "POST",
            &format!("/api/teams/{id}/characters"),
            Some(json!({ "character": created })),
        )
        .await?;
        let path = format!("/api/teams/{id}/characters/{created}");
        gm.ok(
            "PUT",
            &format!("{path}/offer"),
            Some(json!({ "offered": true })),
        )
        .await?;
        gm.ok(
            "PUT",
            &format!("{path}/agendas"),
            Some(json!({ "agendas": agendas })),
        )
        .await?;
    }
    join(&mut ann, &scenario, None).await?;
    let cid_id = join(&mut cid, &scenario, None).await?;
    // Cid brings a Story Point from an earlier scenario (F-53).
    gm.ok(
        "POST",
        &format!("/api/teams/{id}/commands"),
        Some(
            json!({ "command": { "command": "set_story_points", "player": cid_id, "points": 1 } }),
        ),
    )
    .await?;

    Ok(vec![
        format!("accounts: Mother (game master), Ann, Bob, Cid; password \"{PASSWORD}\""),
        "Testlauf Kampagne: Ann plays Nat, Bob plays Ilse Varga; Ann has a draft".into(),
        "Testlauf Szenario: three characters to choose, one a covert android; Ann and Cid have none yet".into(),
    ])
}

async fn account(app: &Router, name: &str, role: &str) -> Result<Client> {
    let mut client = Client::new(app.clone());
    client
        .ok(
            "POST",
            "/api/auth/register",
            Some(json!({ "name": name, "password": PASSWORD, "role": role })),
        )
        .await?;
    Ok(client)
}

/// A character through the wizard: draft, then finish.
async fn finished(client: &mut Client, draft: Value) -> Result<Value> {
    let id = client
        .ok("POST", "/api/characters", Some(json!({})))
        .await?["id"]
        .clone();
    let path = format!("/api/characters/{id}");
    client
        .ok("PUT", &path, Some(json!({ "data": { "draft": draft } })))
        .await?;
    client.ok("POST", &format!("{path}/finish"), None).await?;
    Ok(id)
}

struct Team {
    id: Value,
    token: String,
}

async fn team(gm: &mut Client, name: &str, mode: &str) -> Result<Team> {
    let id = gm
        .ok(
            "POST",
            "/api/teams",
            Some(json!({ "name": name, "mode": mode })),
        )
        .await?["id"]
        .clone();
    let view = gm.ok("GET", &format!("/api/teams/{id}"), None).await?;
    let token = view["invite_token"].as_str().unwrap_or_default().to_owned();
    Ok(Team { id, token })
}

/// Joins with a character or without one; answers the account's id.
async fn join(client: &mut Client, team: &Team, character: Option<&Value>) -> Result<String> {
    client
        .ok(
            "POST",
            &format!("/api/join/{}", team.token),
            Some(json!({ "character": character })),
        )
        .await?;
    let me = client.ok("GET", "/api/auth/me", None).await?;
    Ok(me["id"].to_string())
}

/// Nat the Roughneck with the numbers of the book's example [p. 28].
fn nat() -> Value {
    json!({
        "career": "roughneck",
        "attributes": { "strength": 5, "agility": 3, "wits": 2, "empathy": 4 },
        "skills": { "heavy_machinery": 3, "stamina": 2, "close_combat": 2, "comtech": 1, "observation": 1, "medical_aid": 1 },
        "talent": "true_grit",
        "name": "Nat",
        "appearance": "Kurze rote Haare, Ölflecken an den Händen",
        "agenda": "Niemanden aus der Crew zurücklassen.",
        // A cutting torch and a compression suit, as in the book [p. 33].
        "gear": [{ "line": 0, "option": 0 }, { "line": 2, "option": 1 }],
        "signature_item": "Zerkratzte Thermoskanne"
    })
}

/// A Scientist built by the rules of p. 28.
fn varga() -> Value {
    json!({
        "career": "scientist",
        "attributes": { "strength": 2, "agility": 3, "wits": 5, "empathy": 4 },
        "skills": { "observation": 3, "comtech": 3, "survival": 1, "medical_aid": 1, "mobility": 1, "manipulation": 1 },
        "talent": "analysis",
        "name": "Ilse Varga",
        "appearance": "Schmale Brille, ruhige Stimme",
        "agenda": "Eine Probe von etwas Fremdem sichern, was es auch kostet.",
        // A hand radio and a Personal Medkit.
        "gear": [{ "line": 0, "option": 1 }, { "line": 3, "option": 1 }],
        "signature_item": "Abgegriffenes Notizbuch"
    })
}

/// A prepared character of the game master: name, career, values, an
/// Agenda per Act and the gear the scenario gives [p. 33].
type Prepared = (
    &'static str,
    &'static str,
    Value,
    [&'static str; 3],
    Vec<Value>,
);

fn prepared() -> Vec<Prepared> {
    vec![
        (
            "Kim Hale",
            "pilot",
            json!({
                "attributes": { "strength": 3, "agility": 5, "wits": 3, "empathy": 3 },
                "skills": { "piloting": 3, "ranged_combat": 2, "comtech": 2, "mobility": 1, "observation": 1, "heavy_machinery": 1 },
                "talents": ["reckless"],
                "health": 3
            }),
            [
                "Bring das Shuttle heil zurück.",
                "Finde heraus, wer den Notruf abgesetzt hat.",
                "Rette dich, notfalls allein.",
            ],
            vec![
                json!({ "item": "m4a3_service_pistol", "reloads": 2 }),
                json!({ "item": "irc_mk50_compression_suit", "place": "worn" }),
                json!({ "item": "signal_flare", "count": 3 }),
                json!({ "item": "m314_motion_tracker" }),
            ],
        ),
        (
            "Rosa Ortega",
            "medic",
            json!({
                "attributes": { "strength": 3, "agility": 3, "wits": 3, "empathy": 5 },
                "skills": { "medical_aid": 3, "observation": 2, "mobility": 2, "command": 1, "comtech": 1, "stamina": 1 },
                "talents": ["field_surgeon"],
                "health": 3
            }),
            [
                "Halte alle am Leben.",
                "Untersuche die Verletzten der Station genauer.",
                "Bring die Gewebeprobe hier raus.",
            ],
            vec![
                json!({ "item": "personal_medkit", "count": 2 }),
                json!({ "item": "surgical_kit" }),
                json!({ "item": "naproleve", "count": 2 }),
                json!({ "name": "Gewebeprobe im Kühlbehälter", "weight": "1/2", "place": "stowed" }),
            ],
        ),
        (
            "Victor Lang",
            "company_agent",
            json!({
                "android": true,
                "covert": true,
                "attributes": { "strength": 6, "agility": 3, "wits": 7, "empathy": 2 },
                "skills": { "comtech": 3, "observation": 2, "manipulation": 2, "command": 1, "mobility": 1, "survival": 1 },
                "talents": ["take_control"],
                "health": 6
            }),
            [
                "Sichere die Daten der Forschungsabteilung.",
                "Sorge dafür, dass niemand die Daten löscht.",
                "Die Daten haben Vorrang vor der Crew.",
            ],
            vec![
                json!({ "item": "m4a3_service_pistol", "reloads": 2 }),
                json!({ "item": "seegson_p_dat" }),
            ],
        ),
    ]
}

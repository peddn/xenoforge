#!/usr/bin/env node
// Finds word-for-word passages from the rulebook in repo files.
//
// The repo is public and must not contain text from the book, only numbers
// and own descriptions (CLAUDE.md). This tool compares every line of the
// given files with the book text and reports lines that share a run of at
// least N consecutive words with it.
//
// You need your own copy of the book. Convert it to text outside the repo:
//
//   pdftotext alien-rpg-core-rulebook.pdf ~/alien-book.txt
//   node tools/verbatim-check.mjs --book ~/alien-book.txt
//
// Options:
//   --book <file>   book as plain text (required)
//   --words <n>     shortest run that counts (default 12)
//   <paths...>      files or folders to check (default: data/i18n docs README.md)
//
// Exit code 1 if a line was flagged. Tables with names and numbers can be
// flagged without containing prose; check those by hand.
// No dependencies, Node 22 or newer.

import { readFileSync, readdirSync, statSync } from 'node:fs'
import { extname, join, relative } from 'node:path'

const EXTENSIONS = new Set(['.md', '.json', '.txt', '.js', '.mjs', '.vue', '.rs'])

function parseArgs(argv) {
  const args = { book: null, words: 12, paths: [] }
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i]
    if (arg === '--book') args.book = argv[++i]
    else if (arg === '--words') args.words = Number(argv[++i])
    else if (arg === '--help' || arg === '-h') args.help = true
    else args.paths.push(arg)
  }
  if (args.paths.length === 0) args.paths = ['data/i18n', 'docs', 'README.md']
  return args
}

function words(text) {
  return (
    text
      .toLowerCase()
      .replaceAll('’', "'")
      .replace(/[—–]/g, ' ')
      .match(/[a-z0-9']+/g) ?? []
  )
}

function bookGrams(path, n) {
  // Undo hyphenation at line ends: "elim-\ninated" -> "eliminated".
  const text = readFileSync(path, 'utf8').replace(/-\r?\n\s*/g, '')
  const all = words(text)
  const grams = new Set()
  for (let i = 0; i + n <= all.length; i++) grams.add(all.slice(i, i + n).join(' '))
  return grams
}

function* files(path) {
  const stat = statSync(path)
  if (stat.isDirectory()) {
    for (const name of readdirSync(path).sort()) {
      if (name === 'node_modules' || name.startsWith('.')) continue
      yield* files(join(path, name))
    }
  } else if (EXTENSIONS.has(extname(path))) {
    yield path
  }
}

/** Length of the longest run of words in `line` that also appears in the book. */
function longestRun(line, grams, n) {
  const w = words(line)
  const marked = new Array(w.length).fill(false)
  for (let i = 0; i + n <= w.length; i++) {
    if (grams.has(w.slice(i, i + n).join(' '))) marked.fill(true, i, i + n)
  }
  let best = 0
  let run = 0
  for (const m of marked) {
    run = m ? run + 1 : 0
    best = Math.max(best, run)
  }
  return best
}

const args = parseArgs(process.argv.slice(2))
if (args.help || !args.book || !(args.words > 0)) {
  console.error('usage: node tools/verbatim-check.mjs --book <book.txt> [--words 12] [paths...]')
  process.exit(args.help ? 0 : 2)
}

const grams = bookGrams(args.book, args.words)
let flagged = 0
let checked = 0
for (const root of args.paths) {
  for (const file of files(root)) {
    checked++
    const lines = readFileSync(file, 'utf8').split('\n')
    lines.forEach((line, index) => {
      const run = longestRun(line, grams, args.words)
      if (run >= args.words) {
        flagged++
        const text = line.length > 160 ? `${line.slice(0, 157)}...` : line
        console.log(`${relative('.', file)}:${index + 1} (${run} words): ${text.trim()}`)
      }
    })
  }
}
console.log(`${checked} files checked, ${flagged} lines flagged (runs of ${args.words}+ words)`)
process.exit(flagged > 0 ? 1 : 0)

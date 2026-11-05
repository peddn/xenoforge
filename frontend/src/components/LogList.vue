<script setup>
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

import XDie from '@/ui/XDie.vue'
import XStatus from '@/ui/XStatus.vue'
import { useLogText } from '@/logText'
import { useTeamStore } from '@/stores/team'

// The shared log: every roll with each die and each modification with its
// source (plan 4.2). Everything one command caused forms one block; a
// block the game master took back is marked.
const { t } = useI18n()
const team = useTeamStore()
const { describe, time } = useLogText()
const box = ref(null)

const undone = computed(
  () => new Set(team.log.filter((e) => e.entry.type === 'undone' && e.entry.batch != null).map((e) => e.entry.batch)),
)

// Entries of one command are consecutive; older ones without a batch stand
// alone.
const blocks = computed(() => {
  const list = []
  for (const entry of team.log) {
    const batch = entry.batch ?? entry.id
    let block = list.at(-1)
    if (!block || block.batch !== batch) {
      block = { batch, by: entry.by, at: time(entry.at), note: entry.note, undone: undone.value.has(batch), rows: [] }
      list.push(block)
    }
    block.rows.push({ id: entry.id, ...describe(entry) })
  }
  return list
})

// The log keeps to its newest entry, also when its height changes (the
// decision bar comes and goes), unless someone scrolled up to read.
let atBottom = true

function onScroll() {
  const b = box.value
  atBottom = b.scrollHeight - b.scrollTop - b.clientHeight < 24
}

function keepAtBottom() {
  if (box.value && atBottom) box.value.scrollTop = box.value.scrollHeight
}

watch(
  () => team.log.length,
  async () => {
    await nextTick()
    keepAtBottom()
  },
)

let resize = null
onMounted(() => {
  if (typeof ResizeObserver === 'undefined') return // test environments
  resize = new ResizeObserver(keepAtBottom)
  resize.observe(box.value)
})
onUnmounted(() => resize?.disconnect())
</script>

<template>
  <div
    ref="box"
    class="grid max-h-[60vh] min-h-0 content-start gap-2 overflow-y-auto lg:max-h-none"
    aria-live="polite"
    data-test="log"
    @scroll="onScroll"
  >
    <section
      v-for="block in blocks"
      :key="block.batch"
      :class="['grid gap-1.5 border border-l-[3px] border-line border-l-dim bg-bg px-2.5 py-1.5', block.undone && 'line-through opacity-60']"
      data-test="log-block"
    >
      <header class="text-xs text-dim">
        {{ block.by ?? '' }} · {{ block.at }}
        <XStatus
          v-if="block.undone"
          class="no-underline"
          :text="t('log.undoneMark')"
          data-test="log-undone"
        />
      </header>
      <p
        v-if="block.note"
        class="text-strong"
        data-test="log-note"
      >
        {{ t('log.note', { note: block.note }) }}
      </p>
      <div
        v-for="row in block.rows"
        :key="row.id"
        class="grid gap-1"
        data-test="log-entry"
      >
        <div v-if="row.status || row.text">
          <XStatus
            v-if="row.status"
            :tone="row.status.tone"
            :text="row.status.text"
          />
          {{ row.text }}
        </div>
        <div
          v-if="row.dice"
          class="flex flex-wrap gap-1"
        >
          <XDie
            v-for="(die, i) in row.dice"
            :key="i"
            :die="die"
          />
        </div>
        <div
          v-if="row.detail"
          class="text-xs text-dim"
        >
          {{ row.detail }}
        </div>
      </div>
    </section>
  </div>
</template>

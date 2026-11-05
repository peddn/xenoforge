<script setup>
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import XConfirm from '@/ui/XConfirm.vue'
import XField from '@/ui/XField.vue'
import XStatus from '@/ui/XStatus.vue'
import { itemName, quarters, weightText } from '@/labels'
import { useRulesStore } from '@/stores/rules'

// What a character has [p. 33–34]: the items and where they are, and the
// encumbrance as the server counts it. With `canMove` items are carried,
// put on, stowed, dropped or handed to one of `others`; with `canAdd` the
// game master adds items from the catalog or free ones. Commands go to
// `send` without the actor; the parent sends them, so the same panel works
// in a team and outside one, and answers whether they went through. The
// rules stay with the server.
const props = defineProps({
  actor: { type: Object, required: true },
  encumbrance: { type: Object, default: null },
  canMove: { type: Boolean, default: false },
  canAdd: { type: Boolean, default: false },
  // Other characters to hand items to: { id, name }.
  others: { type: Array, default: () => [] },
  // async (command) => true when the server took it.
  send: { type: Function, default: async () => false },
})
const { t } = useI18n()
const rules = useRulesStore()

const PLACES = ['worn', 'carried', 'stowed']
const placeOf = (p) => p.place ?? 'carried'
const items = computed(() =>
  [...(props.actor.items ?? [])].sort((a, b) => PLACES.indexOf(placeOf(a)) - PLACES.indexOf(placeOf(b))),
)
const entry = (p) => (p.item ? rules.entry('gear', p.item) : null)
const overloaded = computed(() => props.encumbrance && props.encumbrance.carried > props.encumbrance.normal)

function details(p) {
  const e = entry(p)
  const parts = [t('gear.weight', { weight: weightText(e?.weight ?? p.weight) })]
  if (e?.weapon) {
    const w = e.weapon
    parts.push(
      [
        w.bonus ? t('gear.bonus', { bonus: `${w.bonus > 0 ? '+' : ''}${w.bonus}` }) : '',
        w.damage != null ? t('gear.damage', { damage: w.damage }) : '',
        t(`gear.range.${w.range}`),
      ]
        .filter(Boolean)
        .join(', '),
    )
  }
  if (e?.armor) parts.push(t('gear.armor', { rating: e.armor.rating }))
  if (p.reloads) parts.push(t('gear.reloads', p.reloads))
  if (p.supply != null && e?.supply) parts.push(t(`gear.supply.${e.supply.kind}`, { left: p.supply, max: e.supply.max }))
  return parts.join(' · ')
}

// Where an item can go from where it is.
function destinations(p) {
  const list = PLACES.filter((place) => place !== placeOf(p) && (place !== 'worn' || entry(p)?.armor)).map((place) => ({
    value: place,
    label: t(`gear.to.${place}`),
  }))
  for (const other of props.others) {
    list.push({ value: `character:${other.id}`, label: t('gear.toCharacter', { name: other.name }) })
  }
  return list
}

// How many of a stack move; all unless changed.
const counts = ref({})
const countOf = (p) => Math.min(p.count, Math.max(1, Math.trunc(Number(counts.value[p.uid] ?? p.count) || 1)))

function move(p, value) {
  if (!value) return
  const to = value.startsWith('character:') ? { character: value.slice('character:'.length) } : value
  const command = { command: 'move_item', uid: p.uid, to }
  // One suit is put on at a time [p. 98].
  const n = to === 'worn' ? 1 : countOf(p)
  if (n < p.count) command.count = n
  props.send(command)
}

// Adding: from the catalog (things that can be carried, F-63) or free.
const free = ref(false)
const search = ref('')
const picked = ref(null)
const name = ref('')
const weight = ref('1')
const count = ref(1)
const reloads = ref(0)
const place = ref('carried')
const WEIGHTS = ['0', '1/4', '1/2', '1', '2', '3', '4']
const matches = computed(() => {
  const query = search.value.trim().toLowerCase()
  if (!query) return []
  return rules
    .list('gear')
    .filter((e) => e.weight != null && rules.name(e.id).toLowerCase().includes(query))
    .slice(0, 8)
})
const pickedEntry = computed(() => (picked.value ? rules.entry('gear', picked.value) : null))
const canAddNow = computed(() => (free.value ? !!name.value.trim() : !!picked.value))

async function add() {
  const item = free.value ? { name: name.value.trim(), weight: weight.value } : { item: picked.value }
  const n = Math.max(1, Math.trunc(Number(count.value) || 1))
  if (n > 1) item.count = n
  if (place.value !== 'carried') item.place = place.value
  const spare = Math.max(0, Math.trunc(Number(reloads.value) || 0))
  if (!free.value && pickedEntry.value?.weapon?.reloads && spare) item.reloads = spare
  // The form stays filled if the server refuses, e.g. as too heavy.
  if (!(await props.send({ command: 'add_item', item }))) return
  picked.value = null
  search.value = ''
  name.value = ''
  count.value = 1
  reloads.value = 0
  place.value = 'carried'
}

function pick(id) {
  picked.value = id
  if (place.value === 'worn' && !pickedEntry.value?.armor) place.value = 'carried'
}
</script>

<template>
  <div
    class="grid gap-2"
    data-test="gear-panel"
  >
    <div class="flex flex-wrap items-center gap-2">
      <span class="text-sm text-dim uppercase">{{ t('gear.title') }}</span>
      <span
        v-if="encumbrance"
        data-test="encumbrance"
      >
        {{ t('gear.load', { carried: quarters(encumbrance.carried), normal: quarters(encumbrance.normal), max: quarters(encumbrance.max) }) }}
      </span>
      <XStatus
        v-if="overloaded"
        tone="warn"
        :text="t('gear.overloaded')"
      />
    </div>
    <ul class="grid gap-2">
      <li
        v-for="p in items"
        :key="p.uid"
        class="grid gap-1 border-l border-line pl-2"
        :data-test="`item-${p.uid}`"
      >
        <span class="flex flex-wrap items-baseline gap-2">
          <span class="text-strong">{{ itemName(p, rules) }}<template v-if="p.count > 1"> × {{ p.count }}</template></span>
          <span class="text-sm text-dim uppercase">{{ t(`gear.place.${placeOf(p)}`) }}</span>
        </span>
        <span class="text-sm text-dim">{{ details(p) }}</span>
        <span
          v-if="canMove"
          class="flex flex-wrap items-center gap-2"
        >
          <input
            v-if="p.count > 1"
            :value="countOf(p)"
            type="number"
            min="1"
            :max="p.count"
            class="w-16"
            :aria-label="t('gear.count')"
            @input="counts[p.uid] = $event.target.value"
          >
          <select
            :value="''"
            :aria-label="t('gear.moveTo')"
            :data-test="`move-${p.uid}`"
            @change="move(p, $event.target.value); $event.target.value = ''"
          >
            <option
              value=""
              disabled
            >
              {{ t('gear.moveTo') }}
            </option>
            <option
              v-for="d in destinations(p)"
              :key="d.value"
              :value="d.value"
            >
              {{ d.label }}
            </option>
          </select>
          <XConfirm
            size="small"
            variant="danger"
            :label="t('gear.drop')"
            :question="t('gear.dropConfirm', { name: itemName(p, rules) })"
            :data-test="`drop-${p.uid}`"
            @confirm="move(p, 'dropped')"
          />
        </span>
      </li>
      <li
        v-if="!items.length"
        class="text-dim"
      >
        –
      </li>
    </ul>
    <fieldset
      v-if="canAdd"
      class="grid gap-2 border border-line p-3"
      data-test="add-item"
    >
      <legend class="text-sm text-dim uppercase">
        {{ t('gear.add') }}
      </legend>
      <div class="flex flex-wrap items-center gap-3">
        <label class="flex items-center gap-2">
          <input
            v-model="free"
            type="radio"
            :value="false"
          >
          {{ t('gear.fromCatalog') }}
        </label>
        <label class="flex items-center gap-2">
          <input
            v-model="free"
            type="radio"
            :value="true"
            data-test="add-free"
            @change="place === 'worn' && (place = 'carried')"
          >
          {{ t('gear.free') }}
        </label>
      </div>
      <template v-if="!free">
        <XField :label="t('gear.search')">
          <input
            v-model="search"
            data-test="gear-search"
          >
        </XField>
        <div
          v-if="matches.length"
          class="flex flex-wrap gap-1"
        >
          <XButton
            v-for="e in matches"
            :key="e.id"
            size="small"
            :aria-pressed="picked === e.id"
            :data-test="`pick-${e.id}`"
            @click="pick(e.id)"
          >
            {{ rules.name(e.id) }}
          </XButton>
        </div>
      </template>
      <div
        v-else
        class="flex flex-wrap items-end gap-2"
      >
        <XField :label="t('gear.name')">
          <input
            v-model="name"
            maxlength="60"
            data-test="free-name"
          >
        </XField>
        <XField :label="t('gear.weightLabel')">
          <select v-model="weight">
            <option
              v-for="w in WEIGHTS"
              :key="w"
              :value="w"
            >
              {{ weightText(w) }}
            </option>
          </select>
        </XField>
      </div>
      <div class="flex flex-wrap items-end gap-2">
        <XField :label="t('gear.count')">
          <input
            v-model="count"
            type="number"
            min="1"
            class="w-16"
          >
        </XField>
        <XField
          v-if="!free && pickedEntry?.weapon?.reloads"
          :label="t('gear.reloadsLabel')"
        >
          <input
            v-model="reloads"
            type="number"
            min="0"
            class="w-16"
          >
        </XField>
        <XField :label="t('gear.where')">
          <select v-model="place">
            <option
              v-for="pl in PLACES.filter((x) => x !== 'worn' || (!free && pickedEntry?.armor))"
              :key="pl"
              :value="pl"
            >
              {{ t(`gear.place.${pl}`) }}
            </option>
          </select>
        </XField>
        <XButton
          :disabled="!canAddNow"
          data-test="add-go"
          @click="add"
        >
          {{ t('gear.addGo') }}
        </XButton>
      </div>
    </fieldset>
  </div>
</template>

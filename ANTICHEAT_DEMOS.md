# Anti-cheat demos

Console snippets for triggering each rule in `backend/src/services/anticheat.rs`.
UI snippets expose `window.__stopCheat()` to disarm.

To unban yourself between runs:
`update profiles set is_banned = false where id = '<your uuid>';`

| # | Rule         | What happens         | Where                 |
| - | ------------ | -------------------- | --------------------- |
| 1 | T-001 <80ms  | instant ban          | `/play/reaction-time` |
| 2 | T-001 x2     | ban on round 2       | `/play/reaction-time` |
| 3 | T-001 x1     | flag, no ban         | `/play/reaction-time` |
| 4 | V-001        | flag (low variance)  | any (raw WS)          |
| 5 | V-001+T-001  | ban from 2 flags     | any (raw WS)          |
| 6 | Gs P-001     | fast+correct ban     | any (raw WS)          |

---

## 1. T-001 instant — sub-80ms reaction

```js
(() => {
  const zone = document.querySelector('.rt-zone')
  if (!zone) return console.warn('open Reaction Time first')
  let armed = false
  const obs = new MutationObserver(() => {
    const green = zone.classList.contains('zone-green')
    if (green && !armed) { armed = true; setTimeout(() => zone.click(), 30) }
    else if (!green) armed = false
  })
  obs.observe(zone, { attributes: true, attributeFilter: ['class'] })
  window.__stopCheat = () => obs.disconnect()
  console.log('T-001 instant armed (~30ms)')
})()
```

Hit Start. First green flash → ban.

## 2. T-001 escalating — two sub-150ms reactions

Same as #1 but `setTimeout(... , 120)`. Round 1 logs a flag, round 2 bans.

## 3. T-001 single flag

Fires once, then disarms so you finish the rest by hand.

```js
(() => {
  const zone = document.querySelector('.rt-zone')
  if (!zone) return console.warn('open Reaction Time first')
  let used = false
  const obs = new MutationObserver(() => {
    if (used || !zone.classList.contains('zone-green')) return
    used = true
    setTimeout(() => { zone.click(); obs.disconnect() }, 120)
  })
  obs.observe(zone, { attributes: true, attributeFilter: ['class'] })
  console.log('T-001 one-shot armed; play rounds 2–5 normally')
})()
```

No ban. Check the row landed:
`select * from anticheat_log where action = 'flag' order by created_at desc limit 5;`

## 4. V-001 low variance — raw WS, 12 near-identical events

Reaction game caps at 5 rounds, so V-001 needs more events than the UI sends.
This skips the UI and posts straight to the WS.

```js
;(async () => {
  const s = JSON.parse(localStorage.getItem('neuro.session'))
  if (!s?.accessToken) return console.warn('not logged in')
  const start = await fetch('/backend/api/game/gt', {
    method: 'POST', headers: { Authorization: `Bearer ${s.accessToken}` },
  }).then(r => r.json())
  const ws = new WebSocket(
    `${location.protocol === 'https:' ? 'wss:' : 'ws:'}//${location.host}` +
    `/backend/api/game/session/${start.id}/events/ws?token=${encodeURIComponent(s.accessToken)}`
  )
  ws.addEventListener('message', e => {
    try { const m = JSON.parse(e.data); if (m.message_type === 'anticheat') console.log('anticheat:', m) } catch {}
  })
  ws.addEventListener('open', async () => {
    for (let i = 1; i <= 12; i++) {
      ws.send(JSON.stringify({ round: i, event_value: 250 + (i % 3) - 1, client_ts: new Date().toISOString() }))
      await new Promise(r => setTimeout(r, 60))
    }
  })
  window.__cheatWs = ws
})()
```

Round 10 trips V-001. One flag, no ban.

## 5. Two-flag ban — V-001 + T-001

Same as #4, but the last event is below the reaction floor:

```js
for (let i = 1; i <= 11; i++) {
  ws.send(JSON.stringify({ round: i, event_value: 250 + (i % 3) - 1, client_ts: new Date().toISOString() }))
  await new Promise(r => setTimeout(r, 60))
}
ws.send(JSON.stringify({ round: 12, event_value: 90, client_ts: new Date().toISOString() }))
```

Round 12 hits both rules → 2 flags → ban → `/banned`.

## 6. Gs fast+correct — paced cheater

T-001 catches sub-150ms spam, but a smarter script paces itself at 200–300ms
and uses knowledge of the right answer. Five such trials in `gs` instant-ban.

```js
;(async () => {
  const s = JSON.parse(localStorage.getItem('neuro.session'))
  if (!s?.accessToken) return console.warn('not logged in')
  const start = await fetch('/backend/api/game/gs', {
    method: 'POST', headers: { Authorization: `Bearer ${s.accessToken}` },
  }).then(r => r.json())
  const ws = new WebSocket(
    `${location.protocol === 'https:' ? 'wss:' : 'ws:'}//${location.host}` +
    `/backend/api/game/session/${start.id}/events/ws?token=${encodeURIComponent(s.accessToken)}`
  )
  ws.addEventListener('message', e => {
    try { const m = JSON.parse(e.data); if (m.message_type === 'anticheat') console.log('anticheat:', m) } catch {}
  })
  ws.addEventListener('open', async () => {
    for (let i = 1; i <= 5; i++) {
      ws.send(JSON.stringify({ round: i, event_value: 220, correct: true, client_ts: new Date().toISOString() }))
      await new Promise(r => setTimeout(r, 80))
    }
  })
  window.__cheatWs = ws
})()
```

Round 5 trips the gs rule (`P-001`) → ban. Note that 220ms sits above the
T-001 threshold, so this rule is what actually catches it.

---

## Gotchas

- No IP or device fingerprint. Bans are per `user_id` only.
- Ban state lives in `localStorage` (`neuro.banned`), so reloading or
  re-typing the URL still keeps you on `/banned`. Sign out to clear.
- Cleaner reset: clear `neuro.banned` in DevTools → Application → Local
  Storage, sign out, sign back in.

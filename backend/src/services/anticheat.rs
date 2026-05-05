// Anticheat rule proposal
//
// TIMING: response too fast to be human
//   T-001  reaction < 150ms
//   T-002  symbol match score > 120 in 90s
//   T-003  matrix solve < 2s
//   T-004  sequence recall < 200ms after shown
//   T-005  rotation answer < 500ms
//
// PATTERN: scores that don't happen naturally
//   P-001  20+ trials in a row all correct
//   P-002  every hard Gf item correct
//   P-003  scores only go up, never down
//   P-004  exact same metric_value 3+ sessions
//
// VARIANCE: too consistent, humans are noisy
//   V-001  timing std_dev < 5ms over 10+ trials
//   V-002  inter-event gap std_dev < 10ms
//   V-003  literally zero outliers in the distribution
//
// RATE: spamming the system
//   R-001  5+ sessions opened in under a minute
//   R-002  20+ events in 1 second
//   R-003  session done in < 3s
//
// TEMPORAL: messing with timestamps
//   X-001  client_ts ahead of server by > 2s
//   X-002  client_ts goes backward in same session
//   X-003  clock drift > 30s sustained
//
// Escalation:
//   one flag alone                      -> log it, keep going
//   two flags in session                -> ban
//   P-001, P-002, R-001, R-002          -> ban immediately
//   X-001 once                          -> flag + drop that trial
//   X-001 twice                         -> ban


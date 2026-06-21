-- Dezhou table bootstrap seed (moduleId=dezhou, tenant demo-tenant)

INSERT INTO dezhou_table (
  id,
  uuid,
  tenant_id,
  table_code,
  title,
  summary,
  max_seats,
  current_seats,
  small_blind,
  big_blind,
  status,
  created_at,
  updated_at
) VALUES
  (
    'table-nl-1-2',
    '00000000-0000-4000-8000-000000000001',
    'demo-tenant',
    'nl_holdem_1_2',
    'NL Hold''em 1/2',
    'No-limit Texas Hold''em cash table.',
    9,
    0,
    1,
    2,
    'open',
    '2026-01-01T00:00:00Z',
    '2026-01-01T00:00:00Z'
  ),
  (
    'table-nl-2-5',
    '00000000-0000-4000-8000-000000000002',
    'demo-tenant',
    'nl_holdem_2_5',
    'NL Hold''em 2/5',
    'Mid-stakes Texas Hold''em cash table.',
    9,
    3,
    2,
    5,
    'open',
    '2026-01-01T00:00:00Z',
    '2026-01-01T00:00:00Z'
  ),
  (
    'table-tournament-sng',
    '00000000-0000-4000-8000-000000000003',
    'demo-tenant',
    'sng_9max',
    'SNG 9-Max',
    'Single-table sit-and-go tournament.',
    9,
    9,
    10,
    20,
    'full',
    '2026-01-01T00:00:00Z',
    '2026-01-01T00:00:00Z'
  )
ON CONFLICT (id) DO NOTHING;

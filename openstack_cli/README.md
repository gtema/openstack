# New OpenStack CLI

## Facts

- not specifying `-o` will cause a Table output with a CLI normalized and known
  attribute names only. Requesting unsupported fields present in the API
  response is not going to be supported (due to the name convention collision)
- `-o wide` is still considered a human response and
  support normalized/supported names only
- `--plain` may be implemented to output a text form
  table without borders and separators
- `-o XXX[!wide]` is treated as machine response and returns server side names
  and does not support requesting certain fields (they are not known in
  advance).  This decision may be re-evaluated

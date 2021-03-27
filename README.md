# SDMX Blaze

Goals:

1. Survey a list of SDMX endpoints to determine:
   - SDMX Version
   - Accept headers
2. Scrape SDMX by recursing through resources like dataflows
   - Output WARC for all requests
   - Do HEAD requests for datasets to just calculate estimated size of entire data source
   - Then request real data
3. Eventually, parse SDMX data and convert to standard format, probably CSV.
4. Do this all in a performant service that can be potentially scaled

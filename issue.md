```
"org,unicef,data,sdmx)"
Req 4, 421.44861ms, write dur 100.684201ms
Source {
    id: "UNICEF",
    data_content_type: None,
    url: "https://sdmx.data.unicef.org/ws/public/sdmxapi/rest",
    name: "UN Children\'s Fund",
    supports: Some(
        Supports {
            preview: None,
            agencyscheme: None,
            categoryscheme: None,
            codelist: None,
            conceptscheme: None,
            provisionagreement: None,
            structure_specific_data: Some(
                true,
            ),
            datastructure: None,
        },
    ),
    documentation: None,
    headers: None,
    structural_accept: Some(
        Accept {
            supported_accept_headers: [
                "application/xml",
                "application/json",
                "application/vnd.sdmx.structure+xml;version=2.1",
                "application/vnd.sdmx.structure+json;version=1.0.0",
            ],
            denied_accept_headers: [],
        },
    ),
    response_content_types: [
        "application/xml;charset=UTF-8",
        "application/xml;charset=UTF-8",
        "application/xml;charset=UTF-8",
        "application/xml;charset=UTF-8",
    ],
    data_accept: None,
    elapsed: [],
}
"http://data.un.org/WS/rest/dataflow/all/all/latest"
Err(
    "URLs on request and response are not equal",
)
"http://wits.worldbank.org/API/V1/SDMX/V21/rest/dataflow/all/all/latest"
Err(
    "URLs on request and response are not equal",
)
"http://api.worldbank.org/v2/sdmx/rest/dataflow/all/all/latest"
Err(
    "URLs on request and response are not equal",
)
```

Changes in URLS
http -> https
add leading slash

don't be so pedantic with the code

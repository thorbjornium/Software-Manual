# Google Search Operators

## Working

| Search operator | What it does | Example |
| --- | --- | --- |
| `“ ”` | Search for results that mention a word or phrase. | [“steve jobs”](https://www.google.com/search?q=%22steve+jobs%22) |
| `OR` | Search for results related to X or Y. | [jobs OR gates](https://www.google.com/search?&amp;q=jobs+OR+gates) |
| `|` | Same as `OR:` | [jobs | gates](<https://www.google.com/search?q=jobs%7Cgates>) |
| `AND` | Search for results related to X and Y. | [jobs AND gates](https://www.google.com/search?&amp;q=jobs+AND+gates) |
| `-` | Search for results that don’t mention a word or phrase. | [jobs -apple](https://www.google.com/search?q=jobs+-apple) |
| `*` | Wildcard matching any word or phrase. | [steve * apple](https://www.google.com/search?q=%22steve+*+apple%22) |
| `( )` | Group multiple searches. | [(ipad OR iphone) apple](https://www.google.com/search?q=%28ipad+OR+iphone%29+apple) |
| `define:` | Search for the definition of a word or phrase. | [define:entrepreneur](https://www.google.com/search?q=define%3Aentrepreneur) |
| `cache:` | Find the most recent cache of a webpage. | [cache:apple.com](https://webcache.googleusercontent.com/search?q=cache%3Aapple.com) |
| `filetype:` | Search for particular types of files (e.g., PDF). | [apple filetype:pdf](https://www.google.com/search?q=apple+filetype%3Apdf) |
| `ext:` | Same as `filetype:` | [apple ext:pdf](https://www.google.com/search?q=apple+ext%3Apdf) |
| `site:` | Search for results from a particular website. | [site:apple.com](https://www.google.com/search?q=site%3Aapple.com) |
| `related:` | Search for sites related to a given domain. | [related:apple.com](https://www.google.com/search?q=related%3Aapple.com) |
| `intitle:` | Search for pages with a particular word in the title tag. | [intitle:apple](https://www.google.com/search?q=intitle%3Aapple) |
| `allintitle:` | Search for pages with multiple words in the title tag. | [allintitle:apple iphone](https://www.google.com/search?q=allintitle%3Aapple+iphone) |
| `inurl:` | Search for pages with a particular word in the URL. | [inurl:apple](https://www.google.com/search?q=inurl%3Aapple) |
| `allinurl:` | Search for pages with multiple words in the URL. | [allinurl:apple iphone](https://www.google.com/search?q=allinurl%3Aapple+iphone) |
| `intext:` | Search for pages with a particular word in their content. | [intext:apple iphone](https://www.google.com/search?q=intext%3Aapple) |
| `allintext:` | Search for pages with multiple words in their content. | [allintext:apple iphone](https://www.google.com/search?q=allintext%3Aapple+iphone) |
| `weather:` | Search for the weather in a location. | [weather:san francisco](https://www.google.com/search?q=weather%3Asan+francisco) |
| `stocks:` | Search for stock information for a ticker. | [stocks:aapl](https://www.google.com/search?q=stocks%3Aaapl) |
| `map:` | Force Google to show map results. | [map:silicon valley](https://www.google.com/search?q=map%3Asilicon+valley) |
| `movie:` | Search for information about a movie. | [movie:steve jobs](https://www.google.com/search?q=movie%3Asteve+jobs) |
| `in` | Convert one unit to another. | [$329 in GBP](https://www.google.com/search?q=$329+in+GBP) |
| `source:` | Search for results from a particular source in Google News. | [apple source:the_verge](https://www.google.com/search?q=apple+source%3Athe_verge&amp;tbm=nws) |
| `before:` | Search for results from before a particular date. | [apple before:2007-06-29](https://www.google.com/search?q=apple+before%3A2007-06-29) |
| `after:` | Search for results from after a particular date. | [apple after:2007-06-29](https://www.google.com/search?q=apple+after%3A2007-06-29) |

Sidenote.
 You can also use the \_ operator, which acts as a wildcard in Google Autocomplete.

## Unreliable

| Search operator | What it does | Example |
| --- | --- | --- |
| `#..#` | Search within a range of numbers. | [iphone case $50..$60](https://www.google.com/search?q=iphone+case+%2450..%2460) |
| `inanchor:` | Search for pages with backlinks containing specific anchor text. | [inanchor:apple](https://www.google.com/search?q=inanchor%3Aapple) |
| `allinanchor:` | Search for pages with backlinks containing multiple words in their anchor text. | [allinanchor:apple iphone](https://www.google.com/search?q=allinanchor%3Aapple+iphone) |
| `AROUND(X)` | Search for pages with two words or phrases within X words of one another. | [apple AROUND(4) iphone](https://www.google.com/search?q=apple+AROUND%284%29) |
| `loc:` | Find results from a given area. | loc:”san francisco” apple |
| `location:` | Find news from a certain location in Google News. | [location:”san francisco” apple](https://www.google.com/search?q=loc:%22san+francisco%22+apple) |
| `daterange:` | Search for results from a particular date range. | [daterange:11278-13278](https://www.google.com/search?q=steve+jobs+daterange%3A11278-13278) |

## Examples with combinations of operators

Exact Phrase + Content Search + Filetype

```text
“deep learning” allintext:neural networks filetype:pdf before:2020-01-01
```

Exact Phrase + Related Sites Search

```text
“artificial intelligence” related:openai.com after:2022-01-01
```

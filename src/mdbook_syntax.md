# mdBook Syntax  

<!-- toc -->

<br>

## Change browser tab title

````
\{{#title New & improved}}
````

{{#title New & improved}}

## Embedify  

Get [mdBook-embedify](https://github.com/MR-Addict/mdbook-embedify) Embed videos, include banners, scroll-to-top and footer options.

{% embed youtube id="Osqf4oIK0E8" loading="eager" %}

</br>  

## Alerts

Modified and translated version of mdBook-alerts: [https://github.com/horbjorn/rs-mdbook-alerts](https://github.com/horbjorn/rs-mdbook-alerts)

> [!NOTERA]  
> Text

> [!TIPS]
> Annan text

> [!VIKTIGT]  
> ..nånting

> [!VARNING]  
> Se upp! 

> [!FARA]
> Lugn..  

<br>

mdBook's builtin version in html:  

```html
<div class="warning">

This is a bad thing that you should pay attention to.

Warning blocks should be used sparingly in documentation, to avoid "warning
fatigue," where people are trained to ignore them because they usually don't
matter for what they're doing.
</div> 
```

<div class="warning">

This is a bad thing that you should pay attention to.

Warning blocks should be used sparingly in documentation, to avoid "warning
fatigue," where people are trained to ignore them because they usually don't
matter for what they're doing.
</div>  

</br>

## Mermaid

[Mermaid Theme](https://mermaid.js.org/config/theming.html)  

[mdBook-mermaid](https://github.com/badboy/mdbook-mermaid)

```mermaid
%%{
  init: {
    'theme': 'base',
    'themeVariables': {
      'primaryColor': '#bfbfbf',
      'primaryTextColor': '#fff',
      'primaryBorderColor': '#b9b9b9',
      'lineColor': '#abcadf',
      'secondaryColor': '#abcadf',
      'tertiaryColor': '#fff'
    }
  }
}%%

graph LR;

    

    iVentoy(iVentoy)--1.BOOTP--->DHCP(DHCP);
    DHCP--2.PXE-->iVentoy;
    DHCP--3.IP--->Dator(Dator);
    iVentoy--4.Payload-->Dator;
    iVentoy--5.Start-->Dator;
    iVentoy<--shake-->DHCP
     Test(Test)--> TVÅ(TVÅ)
     OST(OST)--> SKINKA(SKINKA)


    click iVentoy href "./iventoy.html"

    style iVentoy fill:#839bac,color:#FFFFFF,stroke-opacity:0.2
    
    style DHCP fill:#d9d9d9,color:#FFFFFF,stroke-opacity:0.2

    style Test fill:#cfd9d8,color:#FFFFFF,stroke-opacity:0.2

    style Dator fill:#cfd9d8,color:#FFFFFF,stroke-opacity:0.2

    style OST fill:#cfd6d9,color:#FFFFFF,stroke-opacity:0.2

    style SKINKA fill:#f3fffd,color:#8c8c8c,stroke-opacity:0.2
    
    
   
```

<details>

<summary>Source code for above</summary>

````
```mermaid
%%{
  init: {
    'theme': 'base',
    'themeVariables': {
      'primaryColor': '#bfbfbf',
      'primaryTextColor': '#fff',
      'primaryBorderColor': '#b9b9b9',
      'lineColor': '#abcadf',
      'secondaryColor': '#abcadf',
      'tertiaryColor': '#fff'
    }
  }
}%%

graph LR;

    

    iVentoy(iVentoy)--1.BOOTP--->DHCP(DHCP);
    DHCP--2.PXE-->iVentoy;
    DHCP--3.IP--->Dator(Dator);
    iVentoy--4.Payload-->Dator;
    iVentoy--5.Start-->Dator;
    iVentoy<--shake-->DHCP
     Test(Test)--> TVÅ(TVÅ)
     OST(OST)--> SKINKA(SKINKA)


    click iVentoy href "./iventoy.html"

    style iVentoy fill:#839bac,color:#FFFFFF,stroke-opacity:0.2
    
    style DHCP fill:#d9d9d9,color:#FFFFFF,stroke-opacity:0.2

    style Test fill:#cfd9d8,color:#FFFFFF,stroke-opacity:0.2

    style Dator fill:#cfd9d8,color:#FFFFFF,stroke-opacity:0.2

    style OST fill:#cfd6d9,color:#FFFFFF,stroke-opacity:0.2

    style SKINKA fill:#f3fffd,color:#8c8c8c,stroke-opacity:0.2
    
```    
   
````

</details>

<br>

<br>

```mermaid
%%{
  init: {
    'theme': 'base',
    'themeVariables': {
      'primaryColor': '#abcadf',
      'primaryTextColor': '#fff',
      'primaryBorderColor': '#abcadf',
      'lineColor': '#fae2b3',
      'secondaryColor': '#89a389',
      'tertiaryColor': '#fff'
    }
  }
}%%
        graph TD
          A[Christmas] -->|Get money| B(Go shopping)
          B --> C{Let me think}
          B --> G[/Another/]
          C ==>|One| D[Laptop]
          C -->|Two| E[iPhone]
          C -->|Three| F[fa:fa-car Car]

style D fill:#cdcdf3,color:#FFFFFF,stroke-opacity:0.2

style E fill:#c8f1f3,color:#FFFFFF,stroke-opacity:0.2

style G fill:#f9e5bc,color:#8c8c8c,stroke-opacity:0.2

style C fill:#839bac,color:#,stroke-opacity:0.2

style B fill:#c7e9d3,color:#FFFFFF,stroke-opacity:0.2

style F fill:#cbd5df,color:#FFFFFF,stroke-opacity:0.2

style Test fill:#dfdfdf,color:#FFFFFF,stroke-opacity:0.2



          subgraph section
            C
            D
            E
            F
            G
          end
          
          
```

<details>

<summary>Source code for above</summary>

````plaintext
```mermaid
%%{
  init: {
    'theme': 'base',
    'themeVariables': {
      'primaryColor': '#abcadf',
      'primaryTextColor': '#fff',
      'primaryBorderColor': '#abcadf',
      'lineColor': '#fae2b3',
      'secondaryColor': '#89a389',
      'tertiaryColor': '#fff'
    }
  }
}%%
        graph TD
          A[Christmas] -->|Get money| B(Go shopping)
          B --> C{Let me think}
          B --> G[/Another/]
          C ==>|One| D[Laptop]
          C -->|Two| E[iPhone]
          C -->|Three| F[fa:fa-car Car]

style D fill:#cdcdf3,color:#FFFFFF,stroke-opacity:0.2

style E fill:#c8f1f3,color:#FFFFFF,stroke-opacity:0.2

style G fill:#f9e5bc,color:#8c8c8c,stroke-opacity:0.2

style C fill:#839bac,color:#,stroke-opacity:0.2

style B fill:#c7e9d3,color:#FFFFFF,stroke-opacity:0.2

style F fill:#cbd5df,color:#FFFFFF,stroke-opacity:0.2

style Test fill:#dfdfdf,color:#FFFFFF,stroke-opacity:0.2



          subgraph section
            C
            D
            E
            F
            G
          end
          
          
```
````
</details>

<br>

<br>

</details>

## Tabs

Get [mdbook-tabs](https://github.com/RustForWeb/mdbook-plugins/tree/main/packages/mdbook-tabs)

```plaintext
{{#tabs }}
{{#tab name="Tab 1" }}
**Tab content 1**
{{#endtab }}
{{#tab name="Tab 2" }}
_Tab content 2_
{{#endtab }}
{{#tab name="Tab 3" }}
~~Tab content 3~~
{{#endtab }}
{{#endtabs }}
```

{{#tabs }}
{{#tab name="Tab 1" }}
**Tab content 1**
{{#endtab }}
{{#tab name="Tab 2" }}
_Tab content 2_
{{#endtab }}
{{#tab name="Tab 3" }}
~~Tab content 3~~
{{#endtab }}
{{#endtabs }}

<br>

```plaintext
{{#tabs global="example" }}
{{#tab name="Tab 1" }}
Some content.
{{#endtab }}
{{#endtabs }}
```

{{#tabs global="example" }}
{{#tab name="Tab 1" }}
Some content.
{{#endtab }}
{{#endtabs }}

## Admonish

Install [admonish](https://github.com/tommilligan/mdbook-admonish)  

Examples:

````plaintext
```admonish
Some text
```
````

````plaintext
```admonish example
example
```
````

```admonish
 Some text
```

```admonish example
example
```

```admonish tldr
abstract, summary, tldr
```

```admonish check
success, check, done
```

```admonish question
question, faq, help
```

```admonish info
info, todo
```

```admonish tip
tip, hint, important
```

```admonish warning
warning, caution, attention
```

```admonish fail
failure, fail, missing
```

```admonish danger
danger, error
```

```admonish bug
bug
```

```admonish quote
quote, cite
```

````plaintext
```admonish quote collapsible=true, title='A title that really <span style="color: #e70073">pops</span>'
To really <b><span style="color: #e70073">grab</span></b> your reader's attention.
```
````

```admonish quote collapsible=true, title='A title that really <span style="color: #e70073">pops</span>'
To really <b><span style="color: #e70073">grab</span></b> your reader's attention.
```

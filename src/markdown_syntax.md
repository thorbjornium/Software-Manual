# Basic Syntax  

<!-- toc -->

{% embed scroll-to-top %}

<br>

## Heading

```
# H1
```

<p style="font-size: 2em; font-weight: bold">H1</p>  

```
## H2  
```

<p style="font-size: 1.5em; font-weight: bold">H2</p>  

```
### H3
```

<p style="font-size: 1.17em; font-weight: bold">H3</p>  

## Heading with custom id

```
## My Great Heading {#custom-id}
```

## My Great Heading {#custom-id}

<br>

## Bold  

```
**bold text**
```

**bold text**

## Italic

```
*italicized text* 
```

*italicized text*  

## Bold and nested italic

```
**This text is *extremely* important**
```

**This text is *extremely* important**

## All bold and italic

```
***All this text is important***
```

***All this text is important***  

## Strikethrough

```
~~The world is flat.~~
```

~~The world is flat.~~

## Emoji

```
That is so funny! :joy: 
```

That is so funny! :joy:  

Might need som additional software to work. [mdbook-emojicodes](https://github.com/blyxyas/mdbook-emojicodes) works for mdBook.

[Here's](https://gist.github.com/horbjorn/cb36d5e32b7b1310f567595acc1da615) a list of emojis.  

## Highlight

```
I need to highlight these ==very important words==.
```

I need to highlight these ==very important words==  

Note: Might not work, else try :

```
I need to highlight these <mark>very important words</mark>
```

I need to highlight these <mark>very important words</mark>

## Subscript

```
H~2~O
```

H~2~O

Note: Might not work, else try :

```
H<sub>2</sub>O
```

H<sub>2</sub>O

## Superscript

```
X^2^
```  

X^2^

Note: Might not work, else try :

```
X<sup>2</sup>
```

X<sup>2</sup>

## Blockquote

```
> blockquote
```

> blockquote

<br>  

```
> blockquote
>
> > nested blockquote
>
```

> blockquote
>
> > nested blockquote
>

<br>

<br>

## Ordered List  

```
1. First item
1. Second item
1. Third item
```  

1. First item
1. Second item
1. Third item  

<br>  

## Unordered List

```
- First item
- Second item
- Third item
```

- First item
- Second item
- Third item

## Nested lists

```
1. First list item
   - First nested list item
     - Second nested list item
```

1. First list item
   - First nested list item
     - Second nested list item

## Code

```
`code` 
```  

`code`


## Fenced Code Block

Three backticks makes a fence. Using a tab or 4 blank spaces instead of backticks also works.

````sh  

```
{
  "firstName": "John",
  "lastName": "Smith",
  "age": 25
}
```

````

```
{
  "firstName": "John",
  "lastName": "Smith",
  "age": 25
}
```

<br>

Language specification is optional.

````
```ruby
require 'redcarpet'
 = Redcarpet.new("Hello World!")
puts .to_
```  
````

```ruby
require 'redcarpet'
 = Redcarpet.new("Hello World!")
puts .to_
```  

## Horizontal Rule

```
---
```

---

<br>

## Link

````
[ Guide](https://www.guide.org)
````

[ Guide](https://www.guide.org)

<br>

```
[an example](http://example.com/ "Title")
```

[an example](http://example.com/ "Title")

<br>

```
Try [Ubuntu][] for your server.

[Ubuntu]: https://ubuntu.com/download/server "Ubuntu Server LTS"
```

Try [Ubuntu][] for your server.

[Ubuntu]: https://ubuntu.com/download/server "Ubuntu Server LTS"

## Image

```
![alt text](https://png.pngtree.com/png-vector/20220629/ourmid/pngtree-cartoon-little-penguin-operating-a-plane-png-image_5567890.png "My image")
```

![alt text](https://png.pngtree.com/png-vector/20220629/ourmid/pngtree-cartoon-little-penguin-operating-a-plane-png-image_5567890.png "My image")

<br>

```
![Logga][1]

[1]: https://commonmark.org/help/images/favicon.png "Creative Commons licensed"
```

![Logga][1]

[1]: https://commonmark.org/help/images/favicon.png "Creative Commons licensed"

### Linked image  

```
[![Markdown image](https://commonmark.org/help/images/favicon.png)](https://commonmark.org)
```

[![Markdown image](https://commonmark.org/help/images/favicon.png)](https://commonmark.org)

## Table

````
| Item              | In Stock | Price |
| :---------------- | :------: | ----: |
| Python Hat        |   Truest | 3.99 |
| SQL Hat           |   True   | 23.99 |
| Codecademy Tee    |  False   | 19.99 |
| Codecademy Hoodie |  False   | 42.9999 |  
````

</br>

| Item              | In Stock | Price |
| :---------------- | :------: | ----: |
| Python Hat        |  Truest  | 3.99 |
| SQL Hat           |   True   | 23.99 |
| Codecademy Tee    |  False   | 19.99 |
| Codecademy Hoodie |  Falsest   | 42.9999 |  

<br>

## Footnote

```
Here's a sentence with a footnote. [^1]

[^1]: This is the footnote.
```

Here's a sentence with a footnote. [^1]

[^1]: This is the footnote.

## Task List

```
- [x] Write the press release
- [ ] Update the website
- [ ] Contact the media
```

- [x] Write the press release
- [ ] Update the website
- [ ] Contact the media

## Inline HTML

### Underline  

```
<u> Text </u> 
```

<u> Text </u>  

### Color

```
<span style="color: #FF69B4;">Why did the tomato turn red?</span>
<span style="color: #00FFFF;">Because it saw the salad dressing!</span>
<span style="color:red;">your text here in red</span>,
<span style="color:#520099;">or make it blue</span>
```

<span style="color: #FF69B4;">Why did the tomato turn red?</span>
<span style="color: #00FFFF;">Because it saw the salad dressing!</span>
<span style="color:red;">your text here in red</span>,
<span style="color:#520099;">or make it blue</span>

### Commentary

```
<!-- ingen kommentar -->  
```

ingen kommentar  

<br>  

### Collapsed section

````
<details>

<summary>Tips for collapsed sections</summary>

### You can add a header

You can add text within a collapsed section. 

You can add an image or a code block, too.

```ruby
   puts "Hello World"
```

</details>
````  

<details>

<summary>Tips for collapsed sections</summary>

### You can add a header

You can add text within a collapsed section.  

You can add an image or a code block, too.

```ruby
   puts "Hello World"
```

</details>

<br>  

### Align

```
&emsp; &emsp;Centrera
```

&emsp; &emsp;Centrera

<br>

```
&ensp; &ensp; &ensp;Centrera
```

&ensp; &ensp; &ensp;Centrera

<br>

```
&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;Centrera
```

&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;Centrera

<br>

### Center align

```
<p style="text-align: center;">Text i mitten 
```

<p style="text-align: center;">Text i mitten  

### Keyboard  

```
<kbd>Ctrl</kbd> <kbd>+</kbd> <kbd>F</kbd>
```

<kbd>Ctrl</kbd> <kbd>+</kbd> <kbd>F</kbd>

### Embed video

#### Local source  

```

<video src="imgburn.mp4" width="100%" height="100%" controls></video>
```

<video src="imgburn.mp4" width="100%" height="100%" controls></video>

#### Interweb source

This from Youtube

```
<iframe
  allowfullscreen
  name="youtube"
  loading="lazy"
  src="https://www.youtube.com/embed/Osqf4oIK0E8"
  style="width: 100%; height: 100%; border: none; aspect-ratio: 16/9; border-radius: 1rem; background: black"
  allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
</iframe>
```  

Replace the video link with your own in the script above. Note: It might break something since not all editors support iframe.

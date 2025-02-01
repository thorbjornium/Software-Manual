# Basic Syntax  

```rust
fn main() {
  let number = 5;
  print!("{}", number);  
  let x = true;
  let e = false;   
}
```

```markdown
**bold text**
- list item
- list item
[x] checked item
- [ ] unchecked item
> blockquote 
```

- [x]

- [ ] unchecked item

```java
public static void main(String[] args) {
  int number = 5;
  System.out.println(number);
}
```

```python
def main() -> None:
  number = 5
  print(number)
```

```c  
int main() {q
  // jskfdfsdf

  int number = 58978969775; 
  printf("%d", number);   
}  
```

```css
body {
  color: red;
  background-color: black;
}
```

```markdown
# H1

```

```html
<html></html>
<mark>bold</mark>
<li>"www.ostechnix.com
-26*.9.'++&"</li>
```

some `code block` with `backticks`

```plaintext
some plain text
www.ostechnix.com
-26*.9.'++&
```

```yaml
key: value
```

```toml
key = "value"
```

```json
{
  "key": "value"
}
```

```xml
<root>
  <child>www.ostechnix.com
-26*.9.'++&</child>
</root>
```

https://www.ostechnix.com  
-26*.9.'++&














<!-- toc -->

<br>

## Heading

```markdown
# H1
```

<p style="font-size: 2em; font-weight: bold">H1</p>  

```markdown
## H2  
```

<p style="font-size: 1.5em; font-weight: bold">H2</p>  

```markdown
### H3
```

<p style="font-size: 1.17em; font-weight: bold">H3</p>  

## Heading with custom id

```markdown
## My Great Heading {#custom-id}
```

## My Great Heading {#custom-id}

<br>

## Bold  

```markdown
**bold text**
```

**bold text**

## Italic

```markdown
*italicized text* 
```

*italicized text*  

## Bold and nested italic

```markdown
**This text is *extremely* important**
```

**This text is *extremely* important**

## All bold and italic

```markdown
***All this text is important***
```

***All this text is important***  

## Strikethrough

```markdown
~~The world is flat.~~
```

~~The world is flat.~~

## Emoji

```markdown
That is so funny! :joy: 
```

That is so funny! :joy:  

Might need som additional software to work. [mdbook-emojicodes](https://github.com/blyxyas/mdbook-emojicodes) works for mdBook.

[Here's](https://gist.github.com/horbjorn/cb36d5e32b7b1310f567595acc1da615) a list of emojis.  

## Highlight

```markdown
I need to highlight these ==very important words==.
```

> [!NOTE]  
> mdBook doesn't support the highlight syntax. Use the html syntax instead.

```html
I need to highlight these <mark>very important words</mark>
```

I need to highlight these <mark>very important words</mark>

## Subscript

```markdown
H~2~O
```

> [!NOTE]  
> mdBook doesn't support the subscript syntax. Use the html syntax instead.

```html
H<sub>2</sub>O
```

H<sub>2</sub>O

## Superscript

```markdown
X^2^
```  

> [!NOTE]  
> mdBook doesn't support the superscript syntax. Use the html syntax instead.

```html
X<sup>2</sup>
```

X<sup>2</sup>

## Blockquote

```markdown
> blockquote
```

> blockquote

<br>  

```markdown
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

```markdown
1. First item
1. Second item
1. Third item
```  

1. First item
1. Second item
1. Third item  

<br>  

## Unordered List

```markdown
- First item
- Second item
- Third item
```

- First item
- Second item
- Third item

## Nested lists

```markdown
1. First list item
   - First nested list item
     - Second nested list item
```

1. First list item
   - First nested list item
     - Second nested list item

## Code

```markdown
`code` 
```  

`code`

## Fenced Code Block

Three backticks makes a fence. Using a tab or 4 blank spaces instead of backticks also works.

````  

```
{
  "firstName": "John",
  "lastName": "Smith",
  "age": 25
}
```

````

```markdown
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

```markdown
---
```

---

<br>

## Link

````markdown
[Guide](https://www.guide.org)
````

[Guide](https://www.guide.org)

<br>

```markdown
[an example](http://example.com/ "Title")
```

[an example](http://example.com/ "Title")

<br>

```markdown
Try [Ubuntu][] for your server.

[Ubuntu]: https://ubuntu.com/download/server "Ubuntu Server LTS"
```

Try [Ubuntu][] for your server.

[Ubuntu]: https://ubuntu.com/download/server "Ubuntu Server LTS"  

## Image  

```markdown  
![alt text](https://png.pngtree.com/png-vector/20220629/ourmid/pngtree-cartoon-little-penguin-operating-a-plane-png-image_5567890.png "My image")
```  

![alt text](https://png.pngtree.com/png-vector/20220629/ourmid/pngtree-cartoon-little-penguin-operating-a-plane-png-image_5567890.png "My image")

<br>

```markdown  
![Logga][1]

[1]: https://commonmark.org/help/images/favicon.png "Creative Commons licensed"
``` 

![Logga][1]

[1]: https://commonmark.org/help/images/favicon.png "Creative Commons licensed"

### Linked image  

```markdown  
[![Markdown image](https://commonmark.org/help/images/favicon.png)](https://commonmark.org)  
```  

[![Markdown image](https://commonmark.org/help/images/favicon.png)](https://commonmark.org)  

## Table  

````markdown
| Item              | In Stock | Price |
| :---------------- | :------: | ----: |
| Python Hat        |   True | 3.99 |
| SQL Hat           |   Truer   | 23.99 |
| Codecademy Tee    |  False   | 19.99 |
| Codecademy Hoodie |  Falser   | 42.9999 |  
````

</br>

| Item              | In Stock | Price |
| :---------------- | :------: | ----: |
| Python Hat        |  True  | 3.99 |
| SQL Hat           |   Truer   | 23.99 |
| Codecademy Tee    |  False   | 19.99 |
| Codecademy Hoodie |  Falser   | 42.9999 |  

<br>

## Footnote

```markdown
Here's a sentence with a footnote. [^1]

[^1]: This is the footnote.
```

Here's a sentence with a footnote. [^1]

[^1]: This is the footnote.

## Task List

```markdown
- [x] Write the press release
- [ ] Update the website
- [ ] Contact the media
```

- [x] Write the press release
- [ ] Update the website
- [ ] Contact the media

## Escaping characters  

You can use a backslash \\ to escape the following characters:

|Character|Name|
|---|:---|
|\ |backslash|  
|` |backtick |  
|* |asterisk|  
|_ |underscore|  
|{ }|curly braces|  
|[ ]|brackets|  
|< >|angle brackets|  
|( )|parentheses|  
|# |pound sign|  
|+ |plus sign|  
|- |minus sign (hyphen)|  
|. |dot|  
|! |exclamation mark|  
|\| |pipe |

Escape backticks with two backticks.  

    ``Use `code` in your Markdown file.``  

Use four backticks to enclose code blocks.

    ````
    ```ruby
    require 'rubygems'
    require 'asciidoctor'
    ```
    ````

## Inline HTML

### Underline  

```html
<u> Text </u> 
```

<u> Text </u>  

### Color

```html
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

```html
<!-- ingen kommentar -->  
```

ingen kommentar  

<br>  

### Collapsed section

````html
<details>

<summary>Tips for collapsed sections</summary>

You can add text within a collapsed section. 

You can add an image or a code block, too.

```ruby
   puts "Hello World"
```

</details>
````  

<details>

<summary>Tips for collapsed sections</summary>

You can add text within a collapsed section.  

You can add an image or a code block, too.

```ruby
   puts "Hello World"
```

</details>

<br>  

### Align

```html
&emsp; &emsp;Centrera
```

&emsp; &emsp;Centrera

<br>

```html
&ensp; &ensp; &ensp;Centrera
```

&ensp; &ensp; &ensp;Centrera

<br>

```html
&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;Centrera
```

&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;Centrera

<br>

### Center align

```html
<p style="text-align: center;">Text i mitten 
```

<p style="text-align: center;">Text i mitten  

### Keyboard  

```html
<kbd>Ctrl</kbd> + <kbd>F</kbd>
```

<kbd>Ctrl</kbd> + <kbd>F</kbd>

### Embed video

#### Local source  

```html
<video src="imgburn.mp4" width="100%" height="100%" controls></video>
```

<video src="imgburn.mp4" width="100%" height="100%" controls></video>

#### Interweb source

```html
<iframe width="100%" height="100%"
src="https://www.youtube.com/embed/tgbNymZ7vqY">
</iframe>  
```

> [!NOTE]  
> height="100%" width="100%" doesn't work in mdBook, so I've changed to width="100%" height="420px" below.

<iframe width="100%" height="420px"
src="https://www.youtube.com/embed/tgbNymZ7vqY">
</iframe>

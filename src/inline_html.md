# Inline HTML

<!-- toc -->

## Underline  

```html
<u> Text </u> 
```

<u> Text </u>  

## Color

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

## Commentary

```html
<!-- ingen kommentar -->  
```

ingen kommentar  

<br>  

## Collapsed section

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

## Align

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

## Center align

```html
<p style="text-align: center;">Text i mitten 
```

<p style="text-align: center;">Text i mitten  

## Keyboard  

```html
<kbd>Ctrl</kbd> + <kbd>F</kbd>
```

<kbd>Ctrl</kbd> + <kbd>F</kbd>

## Embed video

### Local source  

```html
<video src="imgburn.mp4" width="100%" height="100%" controls></video>
```

<video src="imgburn.mp4" width="100%" height="100%" controls></video>

### Interweb source

```html
<iframe width="100%" height="100%"
src="https://www.youtube.com/embed/tgbNymZ7vqY">
</iframe>  
```

> [!NOTE]  
> height="100%" width="100%" doesn't work in mdBook, so I've changed to width="100%" height="420px" below.

<iframe width="95%" height="420px"
src="https://www.youtube.com/embed/tgbNymZ7vqY">
</iframe>

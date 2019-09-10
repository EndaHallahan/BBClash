# BBClash Spec V0.1.1

This document aims to create a standard specification for basic Penclash-flavoured BBCode.

# Goals

Penclash-flavoured BBCode aims to be simple to use, easy to understnd, and quick to write, while still providing users with enough options to be able to achieve most desired formatting needs.

Penclash-flavoured BBCode also aims to be as semantic as possible, giving end applications freedom to style content as needed.

# Basic Syntax

## Tags

Penclash-flavoured BBCode utilizes three types of tags: simple tags (no argument), parameterized tags (one argument), and void tags (no closing tag). These are written as follows:

Simple tag: `[tag][/tag]`

Parameterized tag: `[tag=argument][/tag]`

Void tag: `[tag]`

## Paragraphs

Paragraphs are defined by either **two line breaks** or **a line break and an indent**. A single line break will remain as a single line break, but will be kept in the same paragraph as the text that precedes it. Three line breaks will create a **scene break**.

This is done to allow greater control over the formatting of output by the end application.

# Tag Reference

---

Template:

```
## Tag Name

Tag description.

### Tag:
	`BBCode example. Use {} to indicate user input, e.g. (text contents}.`

### Yields:
	`HTML example. Use {} to indicate user input, e.g. (text contents}.`
```

---

## Center

Center-aligns text.

### Tag:
	
	`[center]{text content}[/center]`

### Yields:
	
	`<div class="center">{text content}</div>


## Right

Right-aligns text.

### Tag:
	`[Right]{text content}[/right]`

### Yields:
	`<div class="right">{text content}</div>`


## Bold and Strong

Emboldens text. The [strong] tag can be nested.

### Tag:
	`[bold]{text content}[/bold]`
	`[strong]{text content}[/strong]`

### Yields:
	`<b>{text content}</b>`
	`<strong>{text content}</strong>`


## Italic and Emphasis

Emphasizes text. The [em] tag can be nested.

### Tag:
	`[i]{text content}[/i]`
	`[em]{text content}[/em]`

### Yields:
	`<i>{text content}</i>`
	`<em>{text content}</em>`


## Underline

Underlines text.

### Tag:
	`[u]{text content}[/u]`

### Yields:
	`<span class="undeline">{text contents}</span>`


## Smallcaps

Sets text to a smallcaps font variant.

### Tag:
	`[smcaps]{text content}[/smcaps]`

### Yields:
	`<span class="smallcaps">{text contents}</span>`


## Strikethrough

Strikes text through.

### Tag:
	`[s]{text content}[/s]`

### Yields:
	`<s>{text contents}</s>`


## Monospace

Sets text to a monospace font variant.

### Tag:
	`[mono]{text content}[/mono]`

### Yields:
	`<span class="monospace">{text contents}</span>`


## Subscript

Subscripts text.

### Tag:
	`[sub]{text content}[/sub]`

### Yields:
	`<sub>{text contents}</sub>`


## Superscript

Superscripts text.

### Tag:
	`[sup]{text content}[/sup]`

### Yields:
	`<sup>{text contents}</sup>`


## Spoiler

Spoilers text.

### Tag:
	`[spoiler]{text content}[/spoiler]`

### Yields:
	`<span class="spoiler">{text contents}</span>`


## Color/Colour

Sets the font colour of text. Accepts hex colour codes prefixed with a '#' and valid web colours. Both spellings (colour/color) are acceptable.

### Tag:
	`[color=#{hex code}]{text content}[/color]`
	`[colour={web colour name}]{text content}[/colour]`

### Yields:
	`span style="color:#{hex code};">{text contents}</span>`
	`span style="color:{web colour name};">{text contents}</span>`


## URL

Creates a hyperlink. Can be named or unnamed, depending on whether or not the url is given as an argument.

### Tag:
	`[url]{web address}[/url]`
	`[url={web address}]{text content}[/url]`

### Yields:
	`<a href="{web address}` rel="nofollow">{web address}</a>`
	`<a href="{web address}` rel="nofollow">{text content}</a>`


## Email

Creates an email hyperlink.

### Tag:
	`[email]{email address}[/email]`

### Yields:
	`<a href="mailto:{email address}`>{email address}</a>`


## Opacity

Sets text opacity. Takes either a percentage or a decimal between 0.0 and 1.0.

### Tag:
	`[opacity=#{opacity value}]{text content}[/color]`

### Yields:
	`<span style="opacity:#{opacity value};">{text contents}</span>`


## Size

Sets text size. Takes either an em or a point value. Values less than 0.5em or above 2.0em will be set to .5 and 2, respectively.

### Tag:
	`[size={size value}]{text content}[/size]`

### Yields:
	`<span style="font-size:#{size value}rem;">{text contents}</span>`


## Header

Creates a header. Header tags range from 1 to 6.

### Tag:
	`[h1]{text content}[/h1]`
	`[h2]{text content}[/h2]`
	`[h6]{text content}[/h6]`

### Yields:
	`<h1>{text content}</h1>`
	`<h2>{text content}</h2>`
	`<h6>{text content}</h6>`


## Image

Inserts an image.

### Tag:
	`[img]{image address}[/img]`

### Yields:
	`<img src="{image address}">`


## Quote

Creates a quote block. Can take the author of the quote as an argument for attribution purposes.

### Tag:
	`[quote]{text content}[/quote]`
	`[quote={quote author}]{text content}[/quote]`

### Yields:
	`<blockquote>{text content}</blockquote>`
	`<blockquote data-author="{quote author}">{text content}</blockquote>`


## Footnote

Creates a footnote. Footnotes are kept inline with the text in their original position under the assumption that formatting will be handled by the application. Footnotes can take an argument for the symbol that will be used to designate them, otherwise they should be sequentially numbered by the application.

### Tag:
	`{text content}[footnote]{footnote content}[/footnote]`
	`{text content}[footnote={footnote symbol}]{footnote content}[/footnote]`

### Yields:
	`{text content}<span class="footnote">{footnote content}</span>`
	`{text content}<span class="footnote" data-symbol = "{footnote symbol}">{footnote content}</span>`


## Pre 

Pre creates a block of preformatted text. Text will be displayed ina  monospaced font and will preserve formatting. 

### Tag:
	`[pre]{text content}[/pre]`

### Yields:
	`<pre>{text content}</pre>`


## Code and Codeblock

Code and codeblock define areas of computer code or markup that should not be formatted. Code defines a single line of text; codeblock allows for multiple lines. Codeblock also allows an argument to define the language of its contents intended for use with syntax highlighting. BBCode tags in Code or Codeblock should be ignored.

### Tag:
	`[code]{text content}[/code]`
	`[codeblock]{text content}[/codeblock]`
	`[codeblock={language}]{text content}[/codeblock]`

### Yields:
	`<code>{text content}</code>`
	`<pre class="codeblock">{text content}</pre>`
	`<pre class="codeblock" data-language="{language}">{text content}</pre>`
	

## Figure

Figure defines self-contained content that can be floated to the right or left. Figure takes one required argument for the floated direction.

### Tag:
	`[figure=right]{content}[/figure]`
	`[figure=left]{content}[/figure]`

### Yields:
	`<figure class="figure-right">{content}</figure>`
	`<figure class="figure-left">{content}</figure>`


## List and List Item

List and its subordinate tag List Item create lists. Lists can be nested. Lists are by default bulleted and unordered. To change the style of bullet for an unordered list, 'circle', 'square', or 'none' may be supplied. to create an ordered list, an argument must be supplied: '1' for numeric, 'a' and 'A' for lower and uppercase albabetic, and 'i' and 'I' for lower and upper case roman numeric.

### Tag:

```
	[list]
		[*]{list item 1}
		[*]{list item 2}
	[/list]
```
```
	[list=circle]
		[*]{list item 1}
		[*]{list item 2}
	[/list]
```
```
	[list=1]
		[*]{list item 1}
		[*]{list item 2}
	[/list]
```

### Yields:

```
<ul>
	<li>{list item 1}</li>
	<li>{list item 2}</li>
</ul>
```
```
<ul style="list-style-type:circle;">
	<li>{list item 1}</li>
	<li>{list item 2}</li>
</ul>
```
```
<ol type="1">
	<li>{list item 1}</li>
	<li>{list item 2}</li>
</ol>
```


## Table, Table Row, Table Cell, and Table Header

Table and its subordinate tags create tables.

### Tag:
TBD

### Yields:
TBD


## Math and Mathblock

Math and Mathblock designate TeX-syntax mathematical formulae to be rendered by the application. Math is for inline code; Mathblock is a block element.

### Tag:
	`[math]{TeX code}[/math]`
	`[mathblock]{TeX code}[/mathblock]`

### Yields:
	`<span class="math_container" data-tex="{TeX code}"></span>`
	`<div class="math_container" data-tex="{TeXCode}"></div>`


## Embed

Embed embeds content from other sources. This should be handled by the end application.

### Tag:
	`[embed]{content address}[/embed]`

### Yields:
	`<div class="embed" data-content="{content address}"></div>`


## Horizontal Rule

Horizontal Rule is a void tag that inserts a horizontal rule.

### Tag:
	`[hr]`

### Yields:
	`<hr>`






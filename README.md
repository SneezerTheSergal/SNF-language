# SNF-language
Sneezer's Note Formatting language is a simple language used to beautify notes
You can write everything in a normal .txt file.
# documentation
`<t>""` will make a title with a header. to define what the title should be, write your title in the "" after the `<t>` tag.

`<i>""` will make a text area. to define what should be in the text area, write your title in the "" after the `<i>` tag.

`<p>""` will make a point. to define what the point should be, write your title in the "" after the `<p>` tag. Points require numbers 1 to 9 after p for correct indentation.

`<e>""` will make an example line. to define what the example should be, write your title in the "" after the `<e>` tag.

`<b>` will make a breakline.

```
<t>"this is the first note"
<i>"in this note i will demonstrate:"
<p1>"that this formatting language can format things like:"
<p2>"points"
<p2>"text fields"
<p2>"titles with headers"
<t>"why?"
<i>"Well, every time I make notes I want them to look perfect, and i want it to be easy to find stuff.
 that's why I made this simple formatting language, which literally just takes a txt file with some text,
 and makes it into something like this."
<e>"example"
```

as of now, you need to have some way of running rust for this to work. there might come something like an interface for this program later.

copy your txt file into `.\inputfiles`. run by doing:
`cargo run -- release .\inputfiles\[your txt file]`

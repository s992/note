# note

Just me trying to learn Rust. Definitely not following best (or even good) practices.

Inspired heavily by [dnote](https://github.com/dnote-io/cli), which is what you should be using if you're looking at this.

## Usage
```
note add <book>
note add <book> -c <note>
note edit <book> <note-index>
note edit <book> <note-index> -c <note>
note ls
note ls <book>
note cat <book> <note-index>
```

## Examples

### Adding a Note

Open `$EDITOR` and fill a note with whatever you type:

`$ note add my-notebook`

Create a note with inline contents:

`$ note add my-notebook -c "heres an inline note"`

### Editing a Note

Open `$EDITOR` for the note:

`$ note edit my-notebook 1`

Replace the contents of the note:

`$ note edit my-notebook 1 -c "replace me!"`

### Listing Notebooks and Notes

View a list of your notebooks:

`$ note ls`

View a list of your notes in a notebook:

`$ note ls my-notebook`

### View a Note

`$ note cat my-notebook 1`

# TV Quotes API

Get over 100,000 quotes from various TV Shows!

https://quotes.jepcd.com

**Show Names are Case Insensitive**

## API Routes

### `GET /quotes/stats`

Get the total number of quotes available.

> https://quotes.jepcd.com/quotes/stats

```json
{ "total": 111173 }
```

### `GET /quotes/shows`

Get all the available shows.

> https://quotes.jepcd.com/quotes/shows

```json
{
  "shows": [
    "How I Met Your Mother",
    "The Middle",
    "New Girl",
    "Suits",
    "3rd Rock from the Sun",
    "Arrested Development",
    "Malcolm in the Middle",
    "Monk",
    "The Fresh Prince of Bel-Air",
    "Parks And Recreation",
    "Home Improvement",
    "Cheers",
    "Modern Family",
    "Seinfeld",
    "The Office",
    "The Goldbergs",
    "Gilmore Girls",
    "Frasier",
    "Breaking Bad",
    "Scrubs",
    "Boy Meets World",
    "Everybody Loves Raymond",
    "The Good Place",
    "Brooklyn Nine-Nine",
    "Everybody Hates Chris",
    "Lucifer",
    "Schitt's Creek",
    "Derry Girls",
    "Friends",
    "Stranger Things",
    "The Golden Girls"
  ]
}
```

### `GET /quotes?show={show}&short={short}`

Get a random quote from a show.
Picks a random show if not specified.

Returns only one-line quotes if short is true. Defaults to false

> https://quotes.jepcd.com/quotes

```json
{
  "show": "Breaking Bad",
  "character": "Water White",
  "text": "We tried to poison you. We tried to poison you because you are an insane, degenerate piece of filth and you deserve to die."
}
```

### `GET /quotes/{number}?show={show}&short={short}`

Get `{number}` random quotes from a show.
Picks random shows if not specified.

Returns only one-line quotes if short is true. Defaults to false

**Max of 10 quotes at a time.**

> https://quotes.jepcd.com/quotes/3

```json
[
  {
    "show": "Suits",
    "character": "Harvey Specter",
    "text": "I'm against having emotions, not against using them."
  },
  {
    "show": "Lucifer",
    "character": "Dan Espinoza",
    "text": "Say you fall in love with a woman who has a cat. What are you gonna do? You accept the cat."
  },
  {
    "show": "Brooklyn Nine-Nine",
    "character": "Jake",
    "text": "Jake: I should probably get some body spray as well. What sounds better, Liquid Moan or Turnpike?\nCaptain Holt: We only have $17 to our name. We can't spend five of it on this.\nJake: I know, but I'm about to see Amy for the first time in forever, and I want to smell like a ... \"hot New Jersey breeze,\" ugh."
  }
]
```

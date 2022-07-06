# description
- learning happens by what happens in the mind of the student more than what is happening in front of them.
- learn by answering concrete questions (brain is working out) instead of reading abstract theory and solutions (brain is eating and digesting).
- answering questions gives a sense of feedback which reading lacks.
- do not mug up, engage and exercise your brain.
- brush up more efficiently before interviews.

# history
- project `schoolbag` - heihrarchical plain text. too less structure.
- projects `journal` and `effortless` - directed acyclic graph of statements + scratch page. too much structure.
- project `learn-by-questions` - answer the question. maybe just right.

# goals
- fast writes of questions from, PR + CI/CD, web form (which creates a PR).
- fast reads from code / web.
- multi choice multi correct questions.
- [common mark](https://commonmark.org/) + (tasklist, tables, footnotes, strikethrough, smart punctuation) storage; readable than json, yaml, toml, xml in native form especially on github.
- html + js; static page serving, no server, while keeping keeping html + js to minimum.
- urls for questions, easy sharing.
- make quiz from search.
- link to git blamed github markdown page.

# approaches
- :cross: rocket
- :cross: zola
- :check: pulldown-cmark
    - `md files` --(read/foreach)--> `string` --(parse)--> `md datastructure` --(validation)--> `is okay?`
        - `not okay markdown` --(output)--> `why and where?`
        - `okay markdown` --(pulldown-cmark html)--> html --(tera)--> html+js --(write)--> public/www --(hosting)--> world --(use)--> solve one-by-one/solve a bunch at a time
- CI/CD is just run the above, if pass -> merge; fail -> do not merge.

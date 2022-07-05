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

# features
- formats
    - multi choice multi correct questions.
    - match the following.
    - fill in the blanks.
- fast writes of questions from PR + CI/CD / web.
- fast reads from code / web.
- common mark storage; readable than json, yaml, toml, xml in native form on github. [strong specification](https://commonmark.org/).
- html + js; static page serving, no server.
- urls for questions, easy sharing.
- make quiz from search.

# approaches
- :cross: rocket
- :cross: zola
- :check: pulldown-cmark
    - `md files` --(read/foreach)--> `string` --(parse)--> `md datastructure` --(validation)--> `is okay?`
        - `not okay markdown` --(output)--> `why and where?`
        - `okay markdown` --(pulldown-cmark html)--> html --(tera)--> html+js --(write)--> public/www --(hosting)--> world --(use)--> solve one-by-one/solve a bunch at a time
- CI/CD is just run the above, if pass -> merge; fail -> do not merge.

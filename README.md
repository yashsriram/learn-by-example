# the problem
- people and teams need to understand one another, in a high dynamic environment like braincorp to function as a one coherent product.
- bugs donot care about team boundaries.
- there is a gap in understanding across teams as experienced by me (i am guessing more people) and surveyed and conveyed in recent all hands talks.
- also seen in tickets like [TST-147](https://braincorporation.atlassian.net/browse/TST-147), [SENSE-75](https://braincorporation.atlassian.net/browse/SENSE-75), [PLATFORM-2801](https://braincorporation.atlassian.net/browse/PLATFORM-2801).

# the solution abstract
- learning happens by [what happens in the mind](https://youtu.be/rhgwIhB58PA) of the student more than what is happening in front of them.
- learn by answering concrete questions :point_right: brain is working out
- learn by reading abstract theory/documents :point_right: brain is eating and digesting.
- do not mug up; engage and exercise your brain :point_right: answer questions.
- answering questions gives feedback, which reading lacks.

# previous work
- this is not my first attempt at knowledge transfer.
- [schoolbag](https://github.com/yashsriram/compost/tree/master/SchoolBag) - heihrarchical plain text. too less structure.
- [journal](https://github.com/yashsriram/effortless/tree/master/deprecated) and [effortless](https://github.com/yashsriram/effortless) - directed acyclic graph of statements + scratch page. too much structure.
- other [forms of knowledge transfer](https://user-images.githubusercontent.com/18350119/177690705-90c4a402-9234-4daf-82b6-38e8867b444e.png) thought about.
- even visualizations ([graphic](https://user-images.githubusercontent.com/18350119/177690941-96514084-1b95-4462-95bf-45228022fc53.png) / [text](https://user-images.githubusercontent.com/18350119/177690944-e9c2f2b3-fb56-4613-a72b-f5fd0d1e235a.png)) are not very engaging.
- __learn-by-questions__ - answer the question. maybe just right.

# the goals
- how do i learn something? (solve problems)
- how can i write down a good question fast with structure? (PR + CI/CD, web form which creates a PR)
- how can i solve a good question fast? (take a quiz by visiting the url)
- how can i solve some good questions fast? (take a quiz by visiting the ... ?)
- how can i read a good question fast? (read the markdown from github url)
- how can i know the author of a question fast? (read the git blame of github url)
- why this format?
    - there is no best format, remember learning happens inside student's head not outside in a quiz software
    - simple and fast to write/validate/read questions
    - simple to evaluate answers
    - fun to implement and maintain
    - any learning tool needs good questions
    - single-correct-multi-choice is a subset of multi-choice-multi-correct
- permalinks for easy sharing.

# maintainance
- [common mark](https://commonmark.org/) + [tasklist, tables, footnotes, strikethrough, smart punctuation] storage; more readable than json, yaml, toml, xml in native form especially on github.
- html + js; static page serving, no server, while keeping keeping html + js to minimum.

# conisdered approaches
- :x: rocket; server not needed
- :x: zola; framework too restrictive and bloated for our purpose
- :heavy_check_mark: pulldown-cmark; used by [mdbook](https://rust-lang.github.io/mdBook/), right for the purpose
    - `md files`
    - --(foreach read)--> `string`
    - --(parse)--> `md datastructure`
    - --(validation)--> `is okay?`
        - `not okay markdown` --(output)--> `why and where?`
        - `okay markdown`
            - --(pulldown-cmark html)--> html
            - --(tera)--> html+js(quiz logic)
            - --(write)-->
            - public/www --(hosting)-->
            - world
- CI/CD is just run the above, if pass -> merge; fail -> do not merge.

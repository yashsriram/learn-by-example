# abstract
- learning happens by [what happens in the mind](https://youtu.be/rhgwIhB58PA) of the student more than what is happening in front of them.
- learn by reading documents :point_right: brain is eating and digesting.
- learn by answering concrete questions :point_right: brain is working out
- do not mug up; engage and exercise your brain :point_right: answer questions.
- quiz gives feedback, which reading lacks.
- brush up more efficiently before interviews.

# previous work
- [schoolbag](https://github.com/yashsriram/compost/tree/master/SchoolBag) - heihrarchical plain text. too less structure.
- [journal](https://github.com/yashsriram/effortless/tree/master/deprecated) and [effortless](https://github.com/yashsriram/effortless) - directed acyclic graph of statements + scratch page. too much structure.
- [some forms of knowledge transfer in the company vs better forms](https://user-images.githubusercontent.com/18350119/177690705-90c4a402-9234-4daf-82b6-38e8867b444e.png) thought about.
- visualization tools ([graphic](https://user-images.githubusercontent.com/18350119/177690941-96514084-1b95-4462-95bf-45228022fc53.png) / [text](https://user-images.githubusercontent.com/18350119/177690944-e9c2f2b3-fb56-4613-a72b-f5fd0d1e235a.png)).
- __learn-by-example__ - answer the question. maybe just right.

# goals
- Lean quiz tool.
- Super easy to contribute, use and maintain.
    - Write question and its answer in markdown, readable in github by itself.
    - CI/CD will gently guide you in case of any problems.
    - If everthing okay you get a URL for your question that you can share.
    - Answer in browser.

# conisdered approaches and final design
- ❌ [rocket](https://rocket.rs/); server not needed
- ❌ [zola](https://www.getzola.org/); ssg is leaner but still a framework; too restrictive and bloated for our purpose
- ✅
    - [common mark](https://commonmark.org/) + [tasklist, tables, footnotes, strikethrough, smart punctuation] storage; more readable than json, yaml, toml, xml in native form especially on github.
    - [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark); md parser; used by [mdbook](https://rust-lang.github.io/mdBook/), right for the purpose
    - html + js; static page serving, no server, while keeping keeping html + js code to minimum.
    - CI/CD; , if pass -> merge; fail -> do not merge.
    - multi choice multi correct questions. why this format?
        - 
            - simple, fast and fun to write/validate/read questions, evaluate answers, implement and maintain
            - gives feedback!
            - there is no best format, remember learning happens inside student's head not outside in a quiz software but this format is missing in brain corp.

# prompt
Which of the following is right about this tool?

# options
- [ ] This is primarily a documentation tool.
- [ ] This is primarily a visualization tool.
- [ ] This tool uses an aws/gcp instance to survive.
- [ ] Anyone in the world can see these questions and answers.
- [x] I write questions and their true answers in markdown which gets turned into a quiz.
- [ ] I can write absolutely anything and it gets turned into a quiz.
- [ ] I cannot use images since this is markdown.
- [x] I can answer questions in a browser.
- [x] I can answer questions in a mobile.
- [x] I can share my question with URL.
- [ ] If you do not know markdown you are doomed.
- [ ] I will have to read html files at some point.

# hint
You are not supposed to know anything here, just guess what you think the tool is about and check.

Don't feel bad if you get an incorrect answer. That's learning 😄👏!

If you finally want to check the truth, press reveal button.

# explaination

This is primarily a (lean) quiz tool.

You can write questions in markdown but you need to follow the minimal structure
```
# prompt
ask something

# options
- [ ] give atleast
- [ ] two options

# hint

# explaination
```
or else CI/CD will ask you to change it.


This is markdown so you can use images, unicode 😀, tables, with github even videos.

![wiki](https://upload.wikimedia.org/wikipedia/commons/3/31/Wiki_logo_Nupedia.jpg)

You can answer questions in any device that has a browser.

This tool creates static html + js pages parsed from markdown and host them using [github pages](https://pages.github.com/).
Only people with access to the repo (and therefore its github pages) can access them. This will be a braincorp org repo so no one outside braincorp will be able access this info

Nope! You can [learn common mark in under 60 seconds here](https://commonmark.org/).

You donot need to read html ever, all questions are stored as markdown which you can read in github itself.

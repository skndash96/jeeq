## JEEQ

#### Get random JEE/NEET questions or a bunch(75, 25 Each of PCM) of questions out of a collection of 0.3 million.
##### Live example @ [here](https://jeeq.herokuapp.com)

- Request at `/q` or `/q?sub=[p|c|m|b]` to get random question from PCMB.
  <br> Response is a JSON stringified array of strings: `[question, subject]`
- Request at `/qp` to get 75 questions
  <br> Response is a JSON stringified array of arrays of strings: `[[question, subject], ...]` indexed as
  - Physics 0-24
  - Chemistry 25-49
  - Maths 50-74


The question string is written in [TeX](https://en.m.wikipedia.org/wiki/TeX) and has to be formatted to make it readable. [MathJAX](https://www.mathjax.org) can be used to format the text on web.

Cons:
  - Existence of questions that require a figure(image)
  - Existence of lame questions like subjective type `What are lactinoids?` (less than 10%)
  - Improper TeX syntax in a few questions (less than 5%)

Datasets are taken from Kaggle.com.
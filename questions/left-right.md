+++

template = "multi-choice-multi-correct.html"

[extra]
options = [
    "Always turn left",
    "Always turn right",
    "Direction of +(c cross t) / (|c| . |t|)",
    "Direction of -(c cross t) / (|c| . |t|)",
]

answer = [
    false,
    false,
    true,
    true,
]

hint = ""

explaination = """
let c = unit vector along theta;
let t = unit vector along omega;
let rotate = (c cross t) / (|c| . |t|)
We should turn in the direction represented by rotate vector to go from c to t.
"""

+++

A [diff drive :robot:](https://en.wikipedia.org/wiki/Differential_wheeled_robot) has a orientation unit vector __c__ and we want to get to align with orientation unit vector __t__.
How should robot turn to do this the fastest?

~~ignore this always~~

soo... `heloo`

```
let A = B;
```


| col 1 | col 2 |
| --- | --- |
| foo | bar |
| foo fasfads | bar dafadfasd |


hello [^privacy]

- [ ] A
- [ ] B
- [ ] C


[^privacy]: There are some privacy concerns. It’s easy enough to keep a shopper’s riding data private from the merchant, but to enforce discount limits, the merchant will have to send sales totals to the server, giving the transit agency the ability to collect substantial information about the shopping habits of its riders. There’s no easy way around this, except to put in place a firewall between the agency and the terminal.

It will probably be necessary for a third party to handle the terminal and servers, providing the service to both merchants and transit agencies (otherwise, each transit agency will have to implement support separately). This third party can be regulated to discard shopping history data beyond the discount window.

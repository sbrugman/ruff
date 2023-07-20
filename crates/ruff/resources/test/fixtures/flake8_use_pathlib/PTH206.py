with Path(filename).open("r") as f:
    data = f.read()

with nothing() as g:
    data = g.hello("foo bar")

with open("r") as f:
    hello = f.read()

with (nothing(), nothing()) as h:
    no_data = h.bla()

with (nothing() as f, nothing() as h):
    pass

with open("r"):
    do_nothing = 123

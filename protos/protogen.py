# owo

rpcs: set[str] = set()

name = input("--- ")


j = input(">>> ")
while j not in [""]:
    rpcs.add(j.capitalize())
    j = input(">>> ")

f = open("out.proto", mode= "+w")

product = ""

product += 'syntax = "proto3";\n'
product += f'package {name}service;\n\n'
product += f'service {name.capitalize()}' + " {\n"
for rpc in rpcs:
    product += f"    rpc {rpc} ({rpc}Request) returns ({rpc}Response);\n"
product += "}\n\n"

for rpc in rpcs:
    product += f"message {rpc}Request" + " {\n"
    product += "}\n"
    product += f"message {rpc}Response" + " {\n"
    product += "}\n\n"

print(product, file=f)


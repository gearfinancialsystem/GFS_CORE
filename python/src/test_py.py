# import lib1 # type: ignore
# import lib2 # type: ignore

# print(lib1.hello_from_lib1())
# print(lib2.hello_from_lib2())
import json

d = json.load(open("documentation/actus-dictionary-terms.json", 'r'))

types_lst = []
for term in d['terms'].keys():
    types_lst.append(d['terms'][term]['type'])

print(set(types_lst))


types_lst = []
for term in d['terms'].keys():
    if d['terms'][term]['type'] == "Enum":
        print("group " + d['terms'][term]['group'] + " - " + term)





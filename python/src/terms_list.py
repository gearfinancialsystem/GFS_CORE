import os
import json

def print_terms(dic):
    for k, v in dic['terms'].items():
        print("%s;%s;%s" %(k, v["group"], v['type']))
        #print("")




data_json = json.load(open("python/actus-dictionary-terms.json", 'rb'))

print_terms(data_json)



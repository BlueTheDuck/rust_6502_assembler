from string import Template
import csv
import io
import sys

size = 0
data_string = ""
src_file = []
append_on = -1
template = Template("static OPCODES: [Opcode;$size] = [\n$list];\n")

with open('data/opcodes_table.csv') as csvfile:
    reader = csv.DictReader(csvfile)
    shall_exit = False
    for row in reader:
        if shall_exit:
            break
        data_string += "\tOpcode{{ name: \"{name}\", code: 0x{code}, mode: AddressingModes::{mode}, size: {size}u16, time: {time}u8  }},\n".format(name=row["NAME"],code=row["CODE"],mode=row["MODE"].upper(),size=row["SIZE"],time=row["TIME"])
        size += 1
        for arg in sys.argv:
            if arg=="--only-one":
                shall_exit = True

with open('src/bin/opcodes.rs',mode="r") as opcodes:
    should_save = True
    for line in opcodes:
        if line=="//#endregion\n" or line=="//#endregion":
            should_save = True
        if should_save:
            src_file.append(line)
        if line=="//#region OPCODES\n":
            should_save = False
            append_on = src_file.__len__(),
            append_on = append_on[0]
    if should_save==False:
        print("Caution!, no closing //#endregion was found")

src_file.insert(append_on,template.substitute(size=size,list=data_string))
print("{} opcodes are going to be written",size)

with open('src/bin/opcodes.rs',mode="w") as opcodes:
    for line in src_file:
        print(line)
        opcodes.write(line);
        
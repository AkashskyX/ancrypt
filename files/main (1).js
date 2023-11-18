const { error } = require('console');
const file_reader = require('fs')


//console.log(" file_reader imported " , file_reader);

const dir = "../data";
const file_name = "data (1).txt"
const file_path = `${dir}/${file_name}`


let processed_data = "ded"


async function ReadFile() {
    try {
        let data = await file_reader.promises.readFile(file_path, 'utf8')

        processed_data = data

    } catch (err) {
        console.log("ERR", err);

    }
}


function split_by_elf() {

    const regex = /^\s*$/gm;

    const separated_elf_intake = processed_data.split(regex)

    return separated_elf_intake




}


async function main() {
    await ReadFile()
    let sep_elf = split_by_elf()

    let lenght = sep_elf.length

    let highest_intake = 0

    let highest_sum_array = []



    for (let i = 0; i < lenght; i++) {

        let sum_of_single_elf = 0



        let single_elf = sep_elf[i].split('\n')

        for (const line of single_elf) {
            const num = parseInt(line)

            if (!isNaN(num)) {
                sum_of_single_elf = num + sum_of_single_elf

            }
        }
            console.log(sum_of_single_elf);

            highest_sum_array.push(sum_of_single_elf)

            if (sum_of_single_elf > highest_intake) {
                highest_intake = sum_of_single_elf

                console.log(i , "is the highest"   , sum_of_single_elf );
            }


    






    }

    
    let sum = 0 
    for (let x = 0; x<3 ; x ++){
    
    let max = Math.max.apply(null, highest_sum_array)
    console.log(max,",max")
    sum = sum + max
    let ind = highest_sum_array.indexOf(max)
   let x =  highest_sum_array.slice(ind)

   console.log(x  , ind, "x");

    }
    console.log(sum, "hi")


    console.log(highest_intake, "🎉");

}



main()






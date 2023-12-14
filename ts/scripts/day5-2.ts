import { readFileSync } from "fs"

// interface SeedRange {
//   start: number
//   end: number
// }

// interface Transformation {
//   (input: SeedRange): SeedRange[]
// }

// function extractSeeds(block: string): SeedRange[] {
//   const seedsString = block.split(":")[1].trim()
//   const seedsNumbers = seedsString.split(" ").map(s => parseInt(s))
//   const seedsIndexes = Array.from({ length: seedsNumbers.length / 2 }).fill(0)
//   const seeds = seedsIndexes.map((_, i) => {
//     return {
//       start: seedsNumbers[i * 2],
//       end: seedsNumbers[i * 2] + seedsNumbers[i * 2 + 1]
//     }
//   })
//   return seeds
// }

// function extractTransformations(blocks: string[]): Transformation[] {
//   const transformations = blocks.map(block => {
//     const lines = block.split("\n").slice(1)
//     const mappers = lines.map(line => {
//       const [to, from, range] = line.split(" ").map(s => parseInt(s))
//       return [to, from, range]
//     })
//     return (input: SeedRange) => {
//       // const overlaps = mappers.filter(([to, from, range]) => {
//       //   return input.start <= from && from < input.end
//       // })
//     }
//   })
// }

function extractSeeds(block: string): number[] {
  const seedsString = block.split(":")[1].trim()
  const seedsNumbers = seedsString.split(" ").map(s => parseInt(s))
  const seedsIndexes = Array.from({ length: seedsNumbers.length / 2 }).fill(0)
  const seeds: number[] = []
  seedsIndexes.forEach((_, i) => {
    const start = seedsNumbers[i * 2]
    const end = seedsNumbers[i * 2] + seedsNumbers[i * 2 + 1]
    for (let j = start; j < end; j++) {
      seeds.push(j)
    }
  })
  return seeds
}

function extractTransformations(blocks: string[]) {
  const transformations = blocks.map((block) => {
    const [title, ...lines] = block.split("\n")
    const mappers = lines.map(line => {
      const [to, from, range] = line.split(" ").map(s => parseInt(s))
      return { to, from, range }
    })
    return (seeds: number[]) => {
      console.log(`appliying ${title}`)
      const results: number[] = []
      for (const seed of seeds) {
        let value = seed
        for (const mapper of mappers) {
          if (value >= mapper.from && value < mapper.from + mapper.range) {
            value = value - mapper.from + mapper.to
          }
        }
        results.push(value)
      }
      return results
    }
  })
  return transformations
}


function main() {
  const filename = process.argv[2]
  const content = readFileSync(filename, "utf-8")
  const blocks = content.split("\n\n")
  const seeds = extractSeeds(blocks[0])
  console.log('seeds', seeds)
  console.log("seeds", seeds.length)
  const tranformations = extractTransformations(blocks.slice(1))
  const result = tranformations.reduce<number[]>((seeds, fn) => fn(seeds), seeds)
  console.log('result', result)
}

main()
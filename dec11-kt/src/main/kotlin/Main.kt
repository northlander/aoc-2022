data class Monkey(
    val items: ArrayDeque<ULong>,
    val operator: String,
    val rightOperator: String,
    val divisibleBy: ULong,
    val monkeyOnTrue: Int,
    val monkeyOnFalse: Int)

fun parse(input: String): List<Monkey> {
    val result = input.lines().chunked(7).map { monkeyStr ->
        Monkey(ArrayDeque(monkeyStr[1].substring(18).split(", ").map { numStr -> numStr.toULong() }),
                            monkeyStr[2].substring(23,24),
                            monkeyStr[2].substring(25),
                            monkeyStr[3].substring(21).toULong(),
                            monkeyStr[4].substring(29).toInt(),
                            monkeyStr[5].substring(30).toInt()
        )
    }

    return result
}

@OptIn(ExperimentalUnsignedTypes::class)
fun part1(input: String): ULong{
    val monkeys = parse(input)

    val inspections = ULongArray(monkeys.size)
    for (round in 1..20) {
        for (monkeyIndex in monkeys.indices) {
            val monkey = monkeys[monkeyIndex]
            while (monkey.items.isNotEmpty()) {
                val itemWorryLevel = monkey.items.removeFirst()
                inspections[monkeyIndex] += 1uL
                val newWorryLevel = when (monkey.operator) {
                    "+" -> itemWorryLevel + if(monkey.rightOperator != "old") monkey.rightOperator.toULong() else itemWorryLevel
                    "*" -> itemWorryLevel * if(monkey.rightOperator != "old") monkey.rightOperator.toULong() else itemWorryLevel
                    else -> {throw Exception("bug!")}
                }

                val reducedWorryLevel = newWorryLevel / 3u
                val newMonkey = if (reducedWorryLevel % monkey.divisibleBy == 0uL) monkey.monkeyOnTrue else monkey.monkeyOnFalse
               // println("Monkey$monkeyIndex - Item $reducedWorryLevel to $newMonkey")
                monkeys[newMonkey].items.addLast(reducedWorryLevel)
            }
        }
    }

    return inspections.sortedDescending().take(2).reduce{ acc, i -> acc * i}
}

@OptIn(ExperimentalUnsignedTypes::class)
fun part2(input: String): ULong{
    val monkeys = parse(input)

    val inspections = ULongArray(monkeys.size)
    val primProduct = monkeys.fold(1uL){acc, m -> m.divisibleBy * acc}
    for (round in 1..10000) {
        for (monkeyIndex in monkeys.indices) {
            val monkey = monkeys[monkeyIndex]
            while (monkey.items.isNotEmpty()) {
                val itemWorryLevel = monkey.items.removeFirst()
                inspections[monkeyIndex] += 1uL
                val newWorryLevel = when (monkey.operator) {
                    "+" -> itemWorryLevel + if(monkey.rightOperator != "old") monkey.rightOperator.toULong() else itemWorryLevel
                    "*" -> itemWorryLevel * if(monkey.rightOperator != "old") monkey.rightOperator.toULong() else itemWorryLevel
                    else -> {throw Exception("bug!")}
                }

                val newMonkey = if (newWorryLevel % monkey.divisibleBy == 0uL) monkey.monkeyOnTrue else monkey.monkeyOnFalse
                // println("Monkey$monkeyIndex - Item $reducedWorryLevel to $newMonkey")
                monkeys[newMonkey].items.addLast(newWorryLevel % primProduct)
            }
        }

        if (round == 1 || round == 20 || round %1000 == 0 ) {
            println("== After round $round")
            monkeys.forEachIndexed { index, _ -> println("Monkey $index inspected items ${inspections[index]} times") }
        }
    }

    return inspections.sortedDescending().take(2).reduce{ acc, i -> acc * i}
}

fun main(args: Array<String>) {

    val input = object{}.javaClass.getResource("input-full.txt").readText()
    println("${part1(input)}")
    println("${part2(input)}")

}
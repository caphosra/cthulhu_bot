//
// Type file for dice-expression-evaluator.
//
declare module "dice-expression-evaluator" {
    export default class DiceExpression {
        constructor(expr: string);
        roll(): { roll: number, diceSums: number[], diceRaw: any };
    }
}

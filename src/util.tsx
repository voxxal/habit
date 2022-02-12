import Snowflake from "nodejs-snowflake";

//@ts-ignore
const uid = new Snowflake({instance_id: 2131});
const nextLevelExp = (level: number) =>
  Math.floor(Math.min(10 * 1.3 ** (level - 1), 1000));
const nextLevelTotalExp = (level: number) => {
  let exp = 0;
  for (let i = 1; i <= level; i++) {
    exp += nextLevelExp(i);
  }
  return Math.floor(exp);
};
const expCurrentLevel = (level: number, exp: number) =>
  nextLevelExp(level) - (nextLevelTotalExp(level) - exp);

const progressToNextLevel = (level: number, exp: number) =>
  expCurrentLevel(level, exp) / nextLevelExp(level);

export { uid, nextLevelExp, nextLevelTotalExp, expCurrentLevel, progressToNextLevel }
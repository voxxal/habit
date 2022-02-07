import dayjs from "dayjs";
import { useEffect, useState } from "react";
import Streak from "./components/Streak";
import { State, StreakData } from "./state";

function App() {
  let [state, setState] = useState<State>({
    experience: 0,
    level: 1,
    streaks: [],
  });
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
  
  const progressToNextLevel = (level: number, exp: number) => expCurrentLevel(level, exp) /
  nextLevelExp(level)

  useEffect(() => {
    setState(
      Object.assign(
        { ...state },
        JSON.parse(localStorage.getItem("habitsSave") || "{}")
      )
    );
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      setState({
        ...state,
        streaks: state.streaks.map((streak) => {
          let newStreak: StreakData = { ...streak };
          if (dayjs().diff(streak.lastCheck, "day") >= 2) {
            newStreak.streak = 0;
          }
          return newStreak;
        }),
      });
    }, 1000 * 60 * 5);
    return () => clearInterval(interval);
  });

  useEffect(() => {
    localStorage.setItem("habitsSave", JSON.stringify(state));
  }, [state]);

  const createNewStreak = () => {
    setState({
      ...state,
      streaks: [
        ...state.streaks,
        {
          name: "",
          streak: 0,
          startTime: dayjs(),
          lastCheck: dayjs().subtract(1, "day"),
        },
      ],
    });
  };

  return (
    <div className="flex flex-col">
      {/* <div
        className=" fixed h-1 bg-purple-500 transition-all duration-700"
        style={{ width: `${(state.experience / (2 ** (state.level / 10) + 100)) * 100}%` }}
      ></div> */}
      <div className="flex w-64 flex-col items-center self-center">
        Level: {state.level}
        <div className="h-2 w-full overflow-hidden rounded-full">
          <div className="h-2 bg-slate-400"></div>
          <div
            className="-mt-2 h-2 bg-purple-500 transition-all duration-700"
            style={{ width: `${ progressToNextLevel(state.level, state.experience) * 100 }%` }}
          ></div>
        </div>
        {expCurrentLevel(state.level, state.experience)} /{" "}
        {nextLevelExp(state.level)}
      </div>
      <div className="flex flex-row flex-wrap">
        {state.streaks.map((streak, i) => (
          <Streak
            key={dayjs(streak.startTime).valueOf()}
            data={streak}
            // STOP USING ANY AHHHHHHH
            rename={(e: any) => {
              let newStreaks = [...state.streaks];
              newStreaks[i].name = e.target.value;
              setState({ ...state, streaks: newStreaks });
            }}
            complete={(e: any) => {
              let newStreaks = [...state.streaks];
              newStreaks[i].streak += 1;
              newStreaks[i].lastCheck = dayjs();
              let exp = state.experience + 3 * (2 / newStreaks[i].streak);
              let levelUp = false; //FIX ME multiple levels
              if (exp >= nextLevelTotalExp(state.level)) levelUp = true;

              setState({
                ...state,
                level: levelUp ? state.level + 1 : state.level,
                experience: Math.floor(exp),
                streaks: newStreaks,
              });
            }}
            remove={(e: any) => {
              let newStreaks = [...state.streaks];
              newStreaks.splice(i, 1);
              setState({
                ...state,
                streaks: newStreaks,
              });
            }}
          />
        ))}
        <div
          className="group m-2 flex h-72 w-64 cursor-pointer flex-col items-center justify-center rounded-xl border-2 border-dashed border-slate-400 p-8  text-slate-400 transition-colors hover:border-blue-400 hover:text-blue-400"
          onClick={createNewStreak}
        >
          <svg
            className="text-slate-400 transition-colors group-hover:text-blue-400"
            width="20"
            height="20"
            fill="currentColor"
            aria-hidden="true"
          >
            <path d="M10 5a1 1 0 0 1 1 1v3h3a1 1 0 1 1 0 2h-3v3a1 1 0 1 1-2 0v-3H6a1 1 0 1 1 0-2h3V6a1 1 0 0 1 1-1Z" />
          </svg>
          Start New Habit
        </div>
      </div>
    </div>
  );
}

export default App;

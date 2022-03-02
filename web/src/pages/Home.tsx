import dayjs from "dayjs";
import Streak from "../components/Streak";
import { State } from "../state";
import { nanoid } from "nanoid";
import { expCurrentLevel, nextLevelExp, progressToNextLevel } from "../util";

function Home({ state, setState }: { state: State; setState: any }) {
  const createNewStreak = () => {
    setState({
      ...state,
      streaks: [
        ...state.streaks,
        {
          id: nanoid(),
          name: "",
          streak: 0,
          startTime: dayjs(),
          lastCheck: dayjs().subtract(1, "day"),
        },
      ],
    });
    console.log(state);
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
            style={{
              width: `${
                progressToNextLevel(state.level, state.experience) * 100
              }%`,
            }}
          ></div>
        </div>
        {expCurrentLevel(state.level, state.experience)} /{" "}
        {nextLevelExp(state.level)}
      </div>
      <div className="flex flex-row flex-wrap">
        {state.streaks.map((streak, i) => (
          <Streak
            key={streak.id}
            data={streak}
            state={state}
            setState={setState}
            index={i}
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

export default Home;
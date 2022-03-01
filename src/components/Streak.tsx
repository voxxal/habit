import dayjs from "dayjs";
import CompleteButton from "./CompleteButton";
import { nextLevelTotalExp } from "../util";
import { MouseEventHandler, useEffect } from "react";
import { State, StreakData } from "../state";
import { Link } from "react-router-dom";

function Streak({
  data,
  state,
  setState,
  index,
}: {
  data: StreakData;
  state: State;
  setState: any; //FIXME bruhv
  index: number;
}) {
  let textarea: HTMLParagraphElement | null;

  useEffect(() => {
    if (textarea) {
      textarea.style.height = "auto";
      textarea.style.height = textarea.scrollHeight + "px";
    }
  }, [data.name]);

  return (
    <Link to={`/streak/${data.id}`} className="m-2 flex h-72 w-64 flex-col rounded-xl bg-slate-100 shadow-md">
      <button
        className="float-right h-9 w-9 self-end rounded-tr-xl bg-slate-200 p-2 align-top font-semibold text-slate-300 transition-colors hover:bg-red-500 hover:text-white"
        onClick={(e: any) => { 
	 e.stopPropagation();
	 e.preventDefault();
	 let newStreaks = [...state.streaks]
	 newStreaks.splice(index, 1); 
	  setState({
            ...state,
            streaks: newStreaks,
          });
        }}
      >
        X
      </button>
      <div className="flex flex-grow flex-col p-8 pt-0">
        <p
          className={`resize-none overflow-y-hidden text-2xl font-bold ${data.name === "" ? "text-gray-400" : ""}`}
          onChange={(e: any) => {
            let newStreaks = [...state.streaks];
            newStreaks[index].name = e.target.value;
            setState({ ...state, streaks: newStreaks });
          }}
          ref={(el) => (textarea = el)}
        >{data.name === "" ? "Unnamed" : data.name}</p>
        <h4 className="my-1">
          Streak:
          <span className="ml-1 inline-flex h-8 w-8 items-center justify-center rounded-full bg-orange-500 p-1 font-semibold text-slate-100">
            {data.streak}
          </span>
        </h4>
        {/* Make it a little more pretty (maybe other components) */}
        <CompleteButton data={data} complete={(e: MouseEventHandler<HTMLButtonElement>) => {
	      //@ts-ignore it does exist though
	      e.stopPropagation();
	      //@ts-ignore it does exist though
	      e.preventDefault();
              let newStreaks = [...state.streaks];
              newStreaks[index].streak += 1;
              newStreaks[index].lastCheck = dayjs();
              let exp = state.experience + 1000 * (2 / newStreaks[index].streak);
              let levelUp = 0; //FIXME multiple levels
              while (exp >= nextLevelTotalExp(state.level + levelUp)) levelUp++;

              setState({
                ...state,
                level: state.level + levelUp,
                experience: Math.floor(exp),
                streaks: newStreaks,
              });
	    }}/>	
      </div>
    </Link>
  );
}

export default Streak;


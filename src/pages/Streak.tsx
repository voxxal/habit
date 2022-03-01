import { useEffect } from "react";
import { State, StreakData } from "../state";
import { useParams } from "react-router-dom";

function Streak({ state, setState }: { state: State; setState: any }) {
  let params = useParams<"streakId">();
  let streakId = params.streakId || "";
  let data: StreakData | null = null;
  data = state.streaks.find((e: StreakData) => e.id === streakId) as StreakData; // TODO redirect for 404s
  let textarea: HTMLTextAreaElement | null;
  // GOT ITTTT LES GO. Its because data is possibly not loaded yet so we have to wait on the state in order to read it
  useEffect(() => {
    (textarea as HTMLTextAreaElement).style.height = "auto";
    (textarea as HTMLTextAreaElement).style.height =
      (textarea as HTMLTextAreaElement).scrollHeight + "px";
  }, [data.name]);

  return (
    <div className="m-4 flex">
      <textarea
        placeholder="Unnamed"
        rows={1}
        className="resize-none overflow-y-hidden bg-transparent text-4xl font-bold"
        maxLength={48}
        value={data.name}
        onChange={(e: any) => {
          let newStreaks = [...state.streaks];
          newStreaks[
            newStreaks.findIndex((e: StreakData) => e.id === streakId)
          ].name = e.target.value; // TODO kinda ugly
          setState({ ...state, streaks: newStreaks });
        }}
        ref={(el) => (textarea = el)}
      ></textarea>
    </div>
  );
}

export default Streak;

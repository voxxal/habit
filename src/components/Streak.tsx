import dayjs from "dayjs";
import { useEffect, useState } from "react";
import { StreakData } from "../state";

function Streak({
  data,
  rename,
  complete,
  remove,
}: {
  data: StreakData;
  rename: any;
  complete: any; //FIXME bruhv
  remove: any;
}) {
  let textarea: HTMLTextAreaElement | null;

  useEffect(() => {
    (textarea as HTMLTextAreaElement).style.height = "auto";
    (textarea as HTMLTextAreaElement).style.height = (textarea as HTMLTextAreaElement).scrollHeight + "px";
  }, [data.name]);

  return (
    <div className="m-2 flex h-72 w-64 flex-col rounded-xl bg-slate-100 shadow-md">
      <button
        className="float-right h-9 w-9 self-end rounded-tr-xl bg-slate-200 p-2 align-top font-semibold text-slate-300 transition-colors hover:bg-red-500 hover:text-white"
        onClick={remove}
      >
        X
      </button>
      <div className="flex flex-grow flex-col p-8 pt-0">
        <textarea
          placeholder="Unnamed"
          rows={1}
          className="resize-none overflow-y-hidden bg-transparent text-2xl font-bold"
          maxLength={48}
          value={data.name}
          onChange={rename}
          ref={(el) => (textarea = el)}
        ></textarea>
        <h4 className="my-1">
          Streak:
          <span className="ml-1 inline-flex h-8 w-8 items-center justify-center rounded-full bg-orange-500 p-1 font-semibold text-slate-100">
            {data.streak}
          </span>
        </h4>
        {/* Make it a little more pretty (maybe other components) */}
        {dayjs().isSame(data.lastCheck, "day") ? (
          <button className="mt-auto h-10 w-full cursor-default items-center rounded-full border-2 border-dashed border-green-500 font-semibold text-green-500">
            <svg
              className="m-auto block h-6 w-6 fill-green-500"
              version="1.1"
              xmlns="http://www.w3.org/2000/svg"
              x="16"
              y="16"
              viewBox="0 0 32 32"
              xmlSpace="preserve"
            >
              <path d="M16,0C7.164,0,0,7.164,0,16s7.164,16,16,16s16-7.164,16-16S24.836,0,16,0z M13.52,23.383    L6.158,16.02l2.828-2.828l4.533,4.535l9.617-9.617l2.828,2.828L13.52,23.383z" />
            </svg>
          </button>
        ) : (
          <button
            className="mt-auto h-10 w-full rounded-full bg-green-500 font-semibold text-white transition-all hover:scale-110 hover:bg-green-600"
            onClick={complete}
          >
            Complete
          </button>
        )}
      </div>
    </div>
  );
}

export default Streak;

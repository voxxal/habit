import dayjs from "dayjs";
import CompleteButton from "./CompleteButton";
import { nextLevelTotalExp } from "../util";
import React, {
  ChangeEvent,
  FormEvent,
  MouseEventHandler,
  useEffect,
} from "react";
import { Action, ActionType, TileData } from "../state";
import { Link } from "react-router-dom";

function Streak({
  data,
  dispatch,
}: {
  data: TileData;
  dispatch: React.Dispatch<Action>;
}) {
  let title: HTMLParagraphElement | null;

  useEffect(() => {
    //TODO not resizing
    if (title) {
      title.style.height = "auto";
      title.style.height = title.scrollHeight + "px";
    }
  }, [data.name]);

  return (
    <Link
      to={`/tile/${data.id}`}
      className="m-2 flex h-72 w-64 flex-col rounded-xl bg-slate-100 shadow-md"
    >
      <button
        className="float-right h-9 w-9 self-end rounded-tr-xl bg-slate-200 p-2 align-top font-semibold text-slate-300 transition-colors hover:bg-red-500 hover:text-white"
        onClick={(e: React.MouseEvent<HTMLButtonElement>) => {
          e.stopPropagation();
          e.preventDefault();
          dispatch({ type: ActionType.DeleteTile, payload: data });
        }}
      >
        X
      </button>
      <div className="flex flex-grow flex-col p-8 pt-0">
        <p
          className={`resize-none overflow-y-hidden text-2xl font-bold ${
            data.name === "" ? "text-gray-400" : ""
          }`}
          ref={(el) => (title = el)}
        >
          {data.name === "" ? "Unnamed" : data.name}
        </p>
        <h4 className="my-1">
          Streak:
          <span className="ml-1 inline-flex h-8 w-8 items-center justify-center rounded-full bg-orange-500 p-1 font-semibold text-slate-100">
            {data.streak.streak(data.startTime)/* TODO: prevent this from crashing*/}
          </span>
        </h4>
        <CompleteButton
          data={data}
          complete={(e: React.MouseEvent<HTMLButtonElement>) => {
            e.preventDefault();
            dispatch({ type: ActionType.CompleteTile, payload: data });
          }}
        />
      </div>
    </Link>
  );
}

export default Streak;

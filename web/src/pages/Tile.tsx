import React, { useContext, useEffect } from "react";
import { ActionType, State, StateContext, TileData } from "../state";
import { useParams } from "react-router-dom";
import CompleteButton from "../components/CompleteButton";

function TilePage() {
  const { state, dispatch } = useContext(StateContext);
  const params = useParams<"tileId">();
  const tileId = params.tileId || "";
  const data: TileData = state.tiles.find(
    (e: TileData) => e.id === tileId //TODO wait on data beforehand.
  ) as TileData; // TODO redirect for 404s
  let textarea: HTMLTextAreaElement | null;
  useEffect(() => {
    (textarea as HTMLTextAreaElement).style.height = "auto";
    (textarea as HTMLTextAreaElement).style.height =
      (textarea as HTMLTextAreaElement).scrollHeight + "px";
  }, [data.name]);
  //TODO back button
  return (
    <div className="m-4 flex flex-col flex-betwe">
      <textarea
        placeholder="Unnamed"
        rows={1}
        className="resize-none overflow-y-hidden bg-transparent text-4xl font-bold"
        maxLength={127}
        value={data.name}
        onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) => {
          dispatch({
            type: ActionType.RenameTile,
            payload: { data, rename: e.target.value },
          });
        }}
        ref={(el) => (textarea = el)}
      ></textarea>
      <CompleteButton
        data={data}
        complete={(e: React.MouseEvent<HTMLButtonElement>) => {
          dispatch({ type: ActionType.CompleteTile, payload: data });
        }}
      />
    </div>
  );
}

export default TilePage;

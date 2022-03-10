import { useEffect } from "react";
import { State, TileData } from "../state";
import { useParams } from "react-router-dom";
import CompleteButton from "../components/CompleteButton";


function TilePage({ state, setState }: { state: State; setState: any }) {
  let params = useParams<"tileId">();
  let tileId = params.tileId || "";
  let data: TileData | null = null;
  data = state.tiles.find((e: TileData) => e.id === tileId) as TileData; // TODO redirect for 404s
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
          let newTiles = [...state.tiles];
          newTiles[
            newTiles.findIndex((e: TileData) => e.id === tileId)
          ].name = e.target.value; // TODO kinda ugly
          setState({ ...state, tiles: newTiles });
        }}
        ref={(el) => (textarea = el)}
      ></textarea>
      <CompleteButton />
    </div>
  );
}

export default TilePage;

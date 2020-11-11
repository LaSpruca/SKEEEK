import * as $ from "jquery";
import { GRID_SIZE, startGame } from "./gameLogic";

export const speed =
  document.location.pathname === "/speed"
    ? 100
    : document.location.pathname === "/ahhh"
    ? 50
    : 200;

const generate_grid = (size: number) =>
  [...Array(size).keys()]
    .map(
      (x) =>
        '<div class="grid-row">' +
        [...Array(size).keys()]
          .map(
            (y) => '<div class="grid-element" id=' + (x * size + y) + "></div>"
          )
          .join("") +
        "</div>"
    )
    .join("");

(() => {
  const speed = $("#speed");
  if (document.location.pathname === "/speed") {
    speed.html("Normal speed");
    speed.attr("href", "/");
    $("#game-over").attr("href", "/speed");
  } else if (document.location.pathname === "/ahhh") {
    speed.html("Normal speed");
    speed.attr("href", "/");
    $("#game-over").attr("href", "/ahhh");
  } else {
    speed.html("Speedi speed");
    speed.attr("href", "/speed");
    $("#game-over").attr("href", "/");
  }

  $(".grid").html(generate_grid(GRID_SIZE));

  startGame();
})();

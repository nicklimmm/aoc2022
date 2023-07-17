(* https://adventofcode.com/2022/day/1 *)

let read_input () =
  let input = open_in "./input/day1.txt" in
  let rec aux acc sub_acc =
    try
      match input_line input with
      | "" -> aux (List.rev sub_acc :: acc) []
      | s -> aux acc (int_of_string s :: sub_acc)
    with End_of_file -> List.rev (sub_acc :: acc)
  in
  aux [] []

let sum = List.fold_left ( + ) 0
let sum_inner = List.map sum
let mx = List.fold_left max 0

let top k l =
  List.sort compare l |> List.rev
  |>
  let rec aux acc k = function
    | [] -> List.rev acc
    | h :: t -> if k = 0 then List.rev acc else aux (h :: acc) (k - 1) t
  in
  aux [] k

let run_a () = read_input () |> sum_inner |> mx |> print_int |> print_newline

let run_b () =
  read_input () |> sum_inner |> top 3 |> sum |> print_int |> print_newline

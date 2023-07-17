type shape = Rock | Paper | Scissors

let shape_of_string = function
  | 'A' | 'X' -> Rock
  | 'B' | 'Y' -> Paper
  | 'C' | 'Z' -> Scissors
  | _ -> failwith "Invalid shape"

let shape_score = function Rock -> 1 | Paper -> 2 | Scissors -> 3

type outcome = Win | Lose | Draw

let outcome_score = function Win -> 6 | Lose -> 0 | Draw -> 3

let outcome_of_string = function
  | 'X' -> Lose
  | 'Y' -> Draw
  | 'Z' -> Win
  | _ -> failwith "Invalid outcome"

let round_outcome opp me =
  match (opp, me) with
  | Rock, Paper | Paper, Scissors | Scissors, Rock -> Win
  | Rock, Scissors | Paper, Rock | Scissors, Paper -> Lose
  | _ -> Draw

let read_input () =
  let input = open_in "./input/day2.txt" in
  let rec read_lines acc =
    try
      let line = input_line input in
      read_lines ((line.[0], line.[2]) :: acc)
    with End_of_file -> List.rev acc
  in
  read_lines []

let round_score opp me = shape_score me + outcome_score (round_outcome opp me)

let me_from opp out =
  match (opp, out) with
  | Rock, Win | Paper, Draw | Scissors, Lose -> Paper
  | Rock, Lose | Paper, Win | Scissors, Draw -> Scissors
  | _ -> Rock

let run_a () =
  List.map
    (fun (opp, me) -> round_score (shape_of_string opp) (shape_of_string me))
    (read_input ())
  |> List.fold_left ( + ) 0 |> print_int |> print_newline

let run_b () =
  List.map
    (fun (opp, out) ->
      let opp_shape = shape_of_string opp in
      let me = me_from opp_shape (outcome_of_string out) in
      round_score opp_shape me)
    (read_input ())
  |> List.fold_left ( + ) 0 |> print_int |> print_newline

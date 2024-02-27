module Cli

using ArgParse

export parse_args

function file_exists(file_path::String)
    if !isfile(file_path)
        throw(ArgumentError("File '$file_path' does not exist."))
    end
    return file_path
end

function parse_commandline()
  s = ArgParseSettings(
    prog="wa_txt2md",
    usage="%(prog)s [options] FILE",
    description="Process Whatsapp's .txt conversations")

  @add_arg_table! s begin
    "FILE"
      arg_type = file_exists
      help = "Whatsapp's txt file to process"
      required = true
    "--name", "-n"
      default = "a Person"
      help = "The conversation's name"
    "--json", "-j"
      action = :store_true
      help = "Enable json generation"
  end

  return parse_args(s)
end

end


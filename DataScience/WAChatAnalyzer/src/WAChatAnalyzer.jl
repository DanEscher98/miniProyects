module WAChatAnalyzer

include("Cli.jl")

function julia_main()::Cint
  args = Cli.parse_commandline()
  file = args["FILE"]
  name = args["name"]
  
  println("FILE: $file, name: $name")
  return 0
end

julia_main()

end

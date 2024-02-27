#include "lib.hpp"
#include "cli.hpp"
#include "utils.hpp"
#include <SDL.h>
#include <glog/logging.h>

const int SCREEN_WIDTH = 640;
const int SCREEN_HEIGHT = 480;

int main(int argc, char* argv[]) {
  google::InitGoogleLogging(argv[0]);
  google::LogToStderr();
  Args& args = Args::getInstance(argc, argv);

  //The window we'll be rendering to
  SDL_Window* window = NULL;
  
  //The surface contained by the window
  SDL_Surface* screenSurface = NULL;

  //Initialize SDL
  if(SDL_Init(SDL_INIT_VIDEO ) < 0) {
    LOG(ERROR) << fmt::format("SDL could not initialize!: %s", SDL_GetError());
  } else {
    //Create window
    window = SDL_CreateWindow( "SDL Tutorial", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, SCREEN_WIDTH, SCREEN_HEIGHT, SDL_WINDOW_SHOWN );
    if( window == NULL ) {
      LOG(ERROR) << fmt::format("Window could not be created!: %s", SDL_GetError());
    } else {

      LOG(INFO) << fmt::format("Window created successfully.");
      //Get window surface
      screenSurface = SDL_GetWindowSurface( window );

      //Fill the surface white
      SDL_FillRect( screenSurface, NULL, SDL_MapRGB( screenSurface->format, 0xFF, 0xFF, 0xFF ) );
      
      //Update the surface
      SDL_UpdateWindowSurface( window );

      //Hack to get window to stay up
      SDL_Event e;
      bool quit = false;
      while( quit == false ){
        while(SDL_PollEvent(&e)){
          if( e.type == SDL_QUIT ) quit = true;
        }
      }
    }
  }

  //Destroy window
  SDL_DestroyWindow( window );

  //Quit SDL subsystems
  SDL_Quit();
  return 0;
}

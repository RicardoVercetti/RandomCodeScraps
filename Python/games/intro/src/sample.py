print("program started..")
import pygame

pygame.init()
screen = pygame.display.set_mode((1280, 720))
clock = pygame.time.Clock()
running = True


while running:
    # poll for events
    # pygame.QUIT event means the user clicked X to close your window
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
    
    screen.fill("purple")

    # render your name here


    # flip the display to put the content to the screen
    pygame.display.flip()

    clock.tick(60)      # limit FPS to 60

pygame.quit()

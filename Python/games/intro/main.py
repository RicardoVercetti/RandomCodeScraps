import pygame
WIDTH = 1280
HEIGHT = 720
pygame.init()

screen = pygame.display.set_mode((WIDTH, HEIGHT))
clock = pygame.time.Clock()
running = True


# task
# draw a circle that will auto hit the edges and get bounced back in the opposite direction
# requirements:
# 1. being able to put a circle
# 2. circle has a width? starting position in the screen/canvas
# 3. has a momentum to one direction
# 4. when the edge of the circle touches(equal to or greater than the edges), the momentum reverses
# 5. this repeats without any external input

# extras
# 1. spawn in random with random size
# 2. spawn several other loons and make them move in random
# 3. when they collide with each other, they should bounce opposite?
# 4. smaller ones get more momentum when hitting against bigger ones
# 5. try acceleration part soon after they hit against one another, then return to their constant speeds

player_pos = pygame.Vector2(0, 0)

while running:
    
    # check for quit event I guess
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            print("pressed the close button!")
            running = False
            continue        # perhaps return right away than going through the rest of the loop

    # screen filling first
    screen.fill("purple")        # maybe try some other color next time

    # draw the content
    pygame.draw.circle(screen, "red", player_pos, 200)      # so the last item is the size

    pygame.display.flip()

    clock.tick(60)

pygame.quit()

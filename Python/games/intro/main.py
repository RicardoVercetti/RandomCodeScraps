import pygame
import random
import time

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


def random_position_in_screen():
    x = random.randrange(WIDTH + 1)
    y = random.randrange(HEIGHT + 1)
    return pygame.Vector2(x, y)

def random_size_of_circle():
    return random.randrange(60, 200)

def change_circle_pos(player_pos):
    # logic for changing the player pos
    return player_pos


player_pos = pygame.Vector2(WIDTH/2, HEIGHT/2)
circle_size = 100

last_pressed = time.time()

def is_time_since_last_pressed():
    now = time.time()
    return now - last_pressed > 0.1 

while running:
    
    # check for quit event I guess
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            print("pressed the close button!")
            running = False
            continue        # perhaps return right away than going through the rest of the loop

    keys = pygame.key.get_pressed()
    # if a specific key is pressed, change position randomly
    if keys[pygame.K_SPACE] and is_time_since_last_pressed():
        player_pos = random_position_in_screen()
        circle_size = random_size_of_circle()
        last_pressed = time.time()

    # screen filling first
    screen.fill("purple")        # maybe try some other color next time

    # draw the content
    pygame.draw.circle(screen, "red", change_circle_pos(player_pos), circle_size)      # so the last item is the size

    pygame.display.flip()

    clock.tick(60)

pygame.quit()

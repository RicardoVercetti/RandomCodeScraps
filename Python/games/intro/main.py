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

class Circle:
    def __init__(self, x_pos, y_pos, x_dir, y_dir, size):
        self.x_pos = x_pos
        self.y_pos = y_pos
        self.x_dir = x_dir
        self.y_dir = y_dir
        self.size = size

    def update_position(self):
        half_size = self.size/2
        if (self.x_pos + half_size) >= WIDTH or (self.x_pos - half_size) <= 0:
            self.x_dir *= -1
        if (self.y_pos + half_size) >= HEIGHT or (self.y_pos - half_size) <= 0:
            self.y_dir *= -1

        self.x_pos += self.x_dir
        self.y_pos += self.y_dir

    @staticmethod
    def random_pos():
        speed = 10
        x_dir = -(speed) if random.randrange(6) % 2 == 0 else speed
        y_dir = -(speed) if random.randrange(6) % 2 == 0 else speed
        # size = random.randrange(100, 120)
        size = 100
        return Circle(random.randrange(WIDTH), random.randrange(HEIGHT), x_dir, y_dir, size)

player_pos = pygame.Vector2(WIDTH/2, HEIGHT/2)
# last_pressed = time.time()
one_circle = Circle.random_pos()


# def is_time_since_last_pressed():
#     now = time.time()
#     return now - last_pressed > 0.1 

while running:
    
    # check for quit event I guess
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            print("pressed the close button!")
            running = False
            continue        # perhaps return right away than going through the rest of the loop


    # screen filling first
    screen.fill("purple")        # maybe try some other color next time

    one_circle.update_position()

    # draw the content
    pygame.draw.circle(screen, "red", pygame.Vector2(one_circle.x_pos, one_circle.y_pos), one_circle.size)      # so the last item is the size
    pygame.display.flip()
    clock.tick(60)

pygame.quit()

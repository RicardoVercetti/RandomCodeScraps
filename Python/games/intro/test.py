"""Just testing out scrap code"""
from dataclasses import dataclass
import pygame

pygame.base.init()

WIDTH = 1200
HEIGH = 720

screen = pygame.display.set_mode((WIDTH, HEIGH))
clock = pygame.time.Clock()
RUNNING = True

@dataclass
class Slate:
    """main bottom slate"""
    length_of_slab: float
    slab_position_vertical: float
    slab_position_horizontal: float
    slab_thickness: float

# the slab position
# length_of_slab = 130
# slab_position_vertical = HEIGH - 30
# slab_position_horizontal = WIDTH/2 - length_of_slab/2
# slab_tickness = 20
length_of_slab = 160
slab = Slate(length_of_slab, HEIGH - 30, WIDTH/2 - length_of_slab/2, 20)


while RUNNING:

    # checking for Quit button
    for event in pygame.event.get():
        if event.type == pygame.constants.QUIT:
            RUNNING = False

    # get input from the keyboard
    key_press = pygame.key.get_pressed()
    if key_press[pygame.K_LEFT]:
        if slab.slab_position_horizontal > 0:
            slab.slab_position_horizontal -= 13
    if key_press[pygame.K_RIGHT]:
        if slab.slab_position_horizontal + slab.length_of_slab < WIDTH:
            slab.slab_position_horizontal += 13

    # the main section
    screen.fill("purple")

    rect = pygame.Rect(slab.slab_position_horizontal, slab.slab_position_vertical, slab.length_of_slab, slab.slab_thickness)
    pygame.draw.rect(screen, "yellow", rect)

    pygame.display.flip()
    clock.tick(60)

pygame.quit()

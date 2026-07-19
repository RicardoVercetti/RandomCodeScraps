package org.example;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.Collections;

public class ComparatorExample {

    static class Student implements Comparable<Student> {
        Integer age;
        String name;
        Student(String name, Integer age) {
            this.name = name;
            this.age = age;
        }

        public String toString() {
            return "name: " + this.name + " | age: " + this.age;
        }


        @Override
        public int compareTo(Student student) {
            int nameCompare = this.name.compareTo(student.name);
            int ageCompare = this.age.compareTo(student.age);

            return nameCompare == 0 ? ageCompare : nameCompare;
        }
    }

    static class StudentAscendingComparator implements Comparator<Student> {
        public int compare(Student s1, Student s2) {
            int nameCompare = s1.name.compareTo(s2.name);
            int ageCompare = s1.age.compareTo(s2.age);
            return nameCompare == 0 ? ageCompare : nameCompare;
        }
    }

    static class StudentDescendingComparator implements Comparator<Student> {
        public int compare(Student s1, Student s2) {
            int nameCompare = s2.name.compareTo(s1.name);
            int ageCompare = s2.age.compareTo(s1.age);
            return nameCompare == 0 ? ageCompare : nameCompare;
        }
    }

    public static void main(String[] args) {
        System.out.println("Started to do things...");

        ArrayList<Student> students = new ArrayList<Student>();
        students.add(new Student("Grace Ancelloti", 34));
        students.add(new Student("Mallorie", 28));
        students.add(new Student("Andrew", 19));
        students.add(new Student("Mallorie", 17));

//        System.out.println("all stds: " + students);
        System.out.println("-- Before sort --");
        for (int i=0; i<students.size(); i++) {
            System.out.println(i + ". " + students.get(i).toString());
        }
        // sorting

        Collections.sort(students, new StudentAscendingComparator());
        System.out.println("-- After Ascending sort --");
        for (int i=0; i<students.size(); i++) {
            System.out.println(i + ". " + students.get(i).toString());
        }

//        Student s1 = new Student("Marzia", 29);
//        Student s2 = new Student("Marzia", 28);
//
//        var comparedValue = s1.compareTo(s2);
//        System.out.println("compared value: " + comparedValue);

        Collections.sort(students, new StudentDescendingComparator());
        System.out.println("-- After Descending sort --");
        for (int i=0; i<students.size(); i++) {
            System.out.println(i + ". " + students.get(i).toString());
        }
    }
}

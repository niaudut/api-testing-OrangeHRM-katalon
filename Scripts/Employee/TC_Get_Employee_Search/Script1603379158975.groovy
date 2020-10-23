import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequestAndVerify(findTestObject('Employee/GET_Employee_Search', [('url') : GlobalVariable.url, ('token') : GlobalVariable.token]))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'data[0].firstName', "Sherina")
WS.verifyElementPropertyValue(response, 'data[0].middleName', "Melodi")
WS.verifyElementPropertyValue(response, 'data[0].lastName', "Darmawan")
WS.verifyElementPropertyValue(response, 'data[0].code', "0009")
WS.verifyElementPropertyValue(response, 'data[0].employeeId', "4")
WS.verifyElementPropertyValue(response, 'data[0].fullName', "Sherina Melodi Darmawan")

WS.verifyElementPropertyValue(response, 'data[1].firstName', "Sherina")
WS.verifyElementPropertyValue(response, 'data[1].lastName', "Darmawan")
WS.verifyElementPropertyValue(response, 'data[1].code', "0010")
WS.verifyElementPropertyValue(response, 'data[1].employeeId', "13")
WS.verifyElementPropertyValue(response, 'data[1].fullName', "Sherina Darmawan")

WS.verifyElementPropertyValue(response, 'data[2].firstName', "john")
WS.verifyElementPropertyValue(response, 'data[2].lastName', "doe")
WS.verifyElementPropertyValue(response, 'data[2].code', "john")
WS.verifyElementPropertyValue(response, 'data[2].employeeId', "18")
WS.verifyElementPropertyValue(response, 'data[2].fullName', "john doe")

//WS.verifyElementPropertyValue(response, 'data[20].firstName', "Febri")
//WS.verifyElementPropertyValue(response, 'data[20].middleName', "Test")
//WS.verifyElementPropertyValue(response, 'data[20].lastName', "Test")
//WS.verifyElementPropertyValue(response, 'data[20].code', "02")
//WS.verifyElementPropertyValue(response, 'data[20].employeeId', "22")
//WS.verifyElementPropertyValue(response, 'data[20].fullName', "Febri Test Test")